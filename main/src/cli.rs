use proc::OclContext;

pub trait OclApp {
    fn ocl_args(self) -> Self;
}

impl<'a, 'b> OclApp for clap::App<'a, 'b> where 'a: 'b {
    fn ocl_args(self) -> Self {
        self
        .arg(
            clap::Arg::with_name("platform_no")
                .help("Platform number")
                .index(1),
        )
        .arg(
            clap::Arg::with_name("device_no")
                .help("Device number")
                .index(2),
        )
        .arg(
            clap::Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List available platform and devices")
                .takes_value(false),
        )
    }
}

pub fn get_ocl_context(matches: &clap::ArgMatches) -> proc::Result<Option<OclContext>> {
    if matches.is_present("list") {
        for (pi, platform) in ocl::Platform::list().into_iter().enumerate() {
            for (di, device) in ocl::Device::list_all(&platform)?.into_iter().enumerate() {
                println!(
                    "Platform [{}]: {}, {}",
                    pi,
                    platform.name()?,
                    platform.version()?,
                );
                println!(
                    "    Device [{}]: {} {}",
                    di,
                    device.vendor()?,
                    device.name()?,
                );
            }
        }
        return Ok(None);
    }

    let platform_no = matches
        .value_of("platform_no")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(0);
    let device_no = matches
        .value_of("device_no")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(0);

    let platform = *ocl::Platform::list().get(platform_no).unwrap();
    let device = *ocl::Device::list_all(&platform)?.get(device_no).unwrap();

    let context = ocl::Context::builder()
        .platform(platform)
        .devices(device)
        .build()?;
    let queue = ocl::Queue::new(&context, device, None)?;

    Ok(Some(OclContext { context, queue }))
}
