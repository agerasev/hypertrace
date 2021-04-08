use crate::{config::*, Entity, EntityType, EntityValue, SizedType, Type, Value};

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
    assert_eq!(<()>::type_id(), dty.id());
    assert_eq!(dty.id(), din.type_of().id());
    assert_eq!(dty.align(&CFG), 1);
    assert_eq!(dty.size(&CFG), 0);
    assert_eq!(din.size(&CFG), 0);
}
