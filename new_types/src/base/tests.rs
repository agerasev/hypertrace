use crate::{config::*, EntityType, EntityValue, SizedType, Type, Value};

const CFG: Config = Config {
    endian: Endian::Little,
    address_width: AddressWidth::X64,
    double_support: true,
};

type DummyType = EntityType<()>;
type DummyValue = EntityValue<()>;

#[test]
fn dummy_type() {
    let (dty, din) = (DummyType::new(), DummyValue::new(()));
    assert_eq!(dty.id(&CFG), din.type_of().id(&CFG));
    assert_eq!(dty.align(&CFG), 1);
    assert_eq!(dty.size(&CFG), 0);
    assert_eq!(din.size(&CFG), 0);
}
