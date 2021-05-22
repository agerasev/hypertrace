use ocl::{Platform, Device, builders::DeviceSpecifier, ProQue};

fn run_for_each_device<F: Fn(Platform, DeviceSpecifier)>(f: F) {
    for platform in Platform::list() {
        println!("Platform: {}", platform.name().unwrap());
        for device in Device::list_all(&platform).unwrap() {
            println!(
                "    Device: {} {}",
                device.vendor().unwrap(),
                device.name().unwrap(),
            );
            f(platform, DeviceSpecifier::Single(device));
        }
    }
}

/// Assume host and all devices are little-endian.
#[test]
fn union_() {
    run_for_each_device(|platform, device_spec| {
        let src = r#"
            typedef union {
                ulong i;
                float f;
            } Union;
            __kernel void test(
                const __global Union *input,
                __global Union *output,
                __global ulong *sizes,
                __global ulong *offsets
            ) {
                uint i = get_global_id(0);
                sizes[i] = sizeof(Union);
                offsets[i] = (ulong)((const __global uchar *)&input[i].f - (const __global uchar *)&input[i]);
                Union t = input[i];
                if (i % 2 == 0) {
                    t.f = 2.0f * t.i;
                } else {
                    t.i = 2 * (ulong)t.f;
                }
                output[i] = t;
            }
        "#;

        let n: usize = 16;

        let pro_que = ProQue::builder()
            .platform(platform)
            .device(device_spec)
            .src(src)
            .dims(n)
            .build()
            .unwrap();

        let input = pro_que.create_buffer::<u64>().unwrap();
        let output = pro_que.create_buffer::<u64>().unwrap();
        let sizes = pro_que.create_buffer::<u64>().unwrap();
        let offsets = pro_que.create_buffer::<u64>().unwrap();

        let kernel = pro_que.kernel_builder("test")
            .arg(&input)
            .arg(&output)
            .arg(&sizes)
            .arg(&offsets)
            .build()
            .unwrap();

        let input_vec = (0..n).map(|i| {
            if i % 2 == 0 {
                i as u64
            } else {
                (i as f32).to_bits() as u64
            }
        }).collect::<Vec<_>>();
        println!("{:?}", input_vec);
        input.write(&input_vec).enq().unwrap();

        unsafe { kernel.enq() }.unwrap();

        let mut output_vec = vec![0; output.len()];
        output.read(&mut output_vec).enq().unwrap();
        println!("{:?}", output_vec);

        let mut sizes_vec = vec![0; output.len()];
        sizes.read(&mut sizes_vec).enq().unwrap();

        let mut offsets_vec = vec![0; output.len()];
        offsets.read(&mut offsets_vec).enq().unwrap();

        pro_que.finish().unwrap();

        for s in sizes_vec {
            assert_eq!(s, 8);
        }

        for o in offsets_vec {
            assert_eq!(o, 0);
        }

        for (i, y) in output_vec.into_iter().enumerate() {
            if i % 2 == 0 {
                assert!(((i as f32) * 2.0 - f32::from_bits(y as u32)).abs() < std::f32::EPSILON);
            } else {
                assert_eq!((i as u64) * 2, y);
            }
        }
    })
}

#[test]
fn zst() {
    run_for_each_device(|platform, device_spec| {
        let src = r#"
            typedef struct {} Empty;
            typedef struct {
                Empty empty;
            } EmptyInside;
            //typedef struct {
            //    uint array[0];
            //} EmptyArray;
            __kernel void test(__global ulong *sizes) {
                if (get_global_id(0) == 0) {
                    Empty empty;
                    sizes[0] = sizeof(empty);
                } else if (get_global_id(0) == 1) {
                    EmptyInside empty_inside;
                    sizes[1] = sizeof(empty_inside);
                } else if (get_global_id(0) == 2) {
                    //EmptyArray empty_array;
                    //sizes[2] = sizeof(empty_array);
                    sizes[2] = 0;
                }
            }
        "#;

        let pro_que = ProQue::builder()
            .platform(platform)
            .device(device_spec)
            .src(src)
            .dims(3)
            .build()
            .unwrap();

        let buffer = pro_que.create_buffer::<u64>().unwrap();

        let kernel = pro_que.kernel_builder("test")
            .arg(&buffer)
            .build()
            .unwrap();

        unsafe { kernel.enq() }.unwrap();

        let mut vec = vec![0u64; buffer.len()];
        buffer.read(&mut vec).enq().unwrap();

        pro_que.finish().unwrap();

        for s in vec {
            assert_eq!(s, 0);
        }
    })
}
