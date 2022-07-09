use strum::{Display, EnumString};

use boulder::{BuildableWithPersianRug, GeneratableWithPersianRug};
use boulder::{Cycle, Inc, Pattern, Some as GSome};
use django_query::{
    filtering::FilterableWithPersianRug, row::IntoRowWithPersianRug,
    sorting::SortableWithPersianRug,
};
use persian_rug::{contextual, Context, Proxy};

#[derive(
    Clone,
    Debug,
    FilterableWithPersianRug,
    SortableWithPersianRug,
    IntoRowWithPersianRug,
    BuildableWithPersianRug,
    GeneratableWithPersianRug,
)]
#[boulder(persian_rug(context = C, access(Alias<C>)))]
#[django(persian_rug(context = C, access(Alias<C>)))]
#[contextual(C)]
pub struct Alias<C: Context + 'static> {
    #[django(exclude)]
    _marker: core::marker::PhantomData<C>,
    #[boulder(default="test-alias", generator=Pattern!("test-alias-{}", Inc(1)))]
    #[django(sort, op(in, contains, icontains, startswith, endswith))]
    pub name: String,
}

// FIXME: we implement IntoRowWithPersianRug to get AsForeignKey derived for us
#[derive(
    Clone,
    Debug,
    FilterableWithPersianRug,
    IntoRowWithPersianRug,
    BuildableWithPersianRug,
    GeneratableWithPersianRug,
)]
#[boulder(persian_rug(context = C, access(Architecture<C>)))]
#[django(persian_rug(context = C, access(Architecture<C>)))]
#[contextual(C)]
pub struct Architecture<C: Context + 'static> {
    #[django(exclude)]
    _marker: core::marker::PhantomData<C>,
    #[boulder(default="test-arch", generator=Pattern!("test-arch-{}", Inc(1)))]
    #[django(op(in, contains, icontains, startswith, endswith))]
    pub name: String,
}

// FIXME: we implement IntoRowWithPersianRug to get AsForeignKey derived for us
#[derive(
    Clone,
    Debug,
    FilterableWithPersianRug,
    IntoRowWithPersianRug,
    BuildableWithPersianRug,
    GeneratableWithPersianRug,
)]
#[boulder(persian_rug(context = C, access(BitWidth<C>)))]
#[django(persian_rug(context = C, access(BitWidth<C>)))]
#[contextual(C)]
pub struct BitWidth<C: Context + 'static> {
    #[django(exclude)]
    _marker: core::marker::PhantomData<C>,
    #[boulder(default=64u64, generator=Cycle::new(vec![32u64,64u64].into_iter()))]
    #[django(op(in))]
    pub width: u64,
}

// FIXME: we implement IntoRowWithPersianRug to get AsForeignKey derived for us
#[derive(
    Clone,
    Debug,
    FilterableWithPersianRug,
    IntoRowWithPersianRug,
    BuildableWithPersianRug,
    GeneratableWithPersianRug,
)]
#[boulder(persian_rug(context = C, access(Core<C>)))]
#[django(persian_rug(context = C, access(Core<C>)))]
#[contextual(C)]
pub struct Core<C: Context + 'static> {
    #[django(exclude)]
    _marker: core::marker::PhantomData<C>,
    #[boulder(default="test-core", generator=Pattern!("test-core-{}", Inc(1)))]
    #[django(op(in, contains, icontains, startswith, endswith))]
    pub name: String,
}

// FIXME: we implement IntoRowWithPersianRug to get AsForeignKey derived for us
#[derive(
    Clone,
    Debug,
    FilterableWithPersianRug,
    IntoRowWithPersianRug,
    BuildableWithPersianRug,
    GeneratableWithPersianRug,
)]
#[boulder(persian_rug(context = C, access(ProcessorFamily<C>)))]
#[django(persian_rug(context = C, access(ProcessorFamily<C>)))]
#[contextual(C)]
pub struct ProcessorFamily<C: Context + 'static> {
    #[django(exclude)]
    _marker: core::marker::PhantomData<C>,
    #[boulder(default="test-processor-family", generator=Pattern!("test-processor-family-{}", Inc(1)))]
    #[django(op(in, contains, icontains, startswith, endswith))]
    pub name: String,
}

// FIXME: Verify: the docs say this is not sortable
// FIXME: Only implementing sortable so that we can do a nested sort on the name key
#[derive(
    Clone,
    Debug,
    FilterableWithPersianRug,
    SortableWithPersianRug,
    IntoRowWithPersianRug,
    BuildableWithPersianRug,
    GeneratableWithPersianRug,
)]
#[boulder(
    persian_rug(
        context = C,
        access(
            DeviceType<C>,
            Alias<C>,
            Architecture<C>,
            BitWidth<C>,
            Core<C>,
            ProcessorFamily<C>
        )
    )
)]
#[django(
    persian_rug(
        context = C,
        access(
            DeviceType<C>,
            Alias<C>,
            Architecture<C>,
            BitWidth<C>,
            Core<C>,
            ProcessorFamily<C>
        )
    )
)]
#[contextual(C)]
pub struct DeviceType<C: Context + 'static> {
    #[boulder(default="test-device-type", generator=Pattern!("test-device-type-{}", Inc(0)))]
    #[django(sort, op(in, contains, icontains, startswith, endswith))]
    pub name: String,
    #[boulder(buildable_with_persian_rug, generatable_with_persian_rug)]
    #[django(traverse, foreign_key = "name")]
    pub architecture: Option<Proxy<Architecture<C>>>,
    #[boulder(buildable_with_persian_rug, generatable_with_persian_rug)]
    #[django(traverse, foreign_key = "name")]
    pub processor: Option<Proxy<ProcessorFamily<C>>>,
    #[boulder(default=Some("test-cpu-model".to_string()), generator=GSome(Pattern!("test-cpu-model-{}", Inc(0))))]
    #[django(op(in, contains, icontains, startswith, endswith))]
    pub cpu_model: Option<String>,
    #[boulder(generatable_with_persian_rug, sequence = 2usize)]
    #[django(traverse, foreign_key = "name")]
    pub aliases: Vec<Proxy<Alias<C>>>,
    #[boulder(buildable_with_persian_rug, generatable_with_persian_rug)]
    #[django(traverse, foreign_key = "width")]
    pub bits: Option<Proxy<BitWidth<C>>>,
    #[boulder(generatable_with_persian_rug,sequence=4usize,sequence_generator=Cycle::new(vec![4usize,8usize,16usize].into_iter()))]
    #[django(traverse, foreign_key = "name")]
    pub cores: Vec<Proxy<Core<C>>>,
    #[boulder(default=Some(4), generator=GSome(Cycle::new(vec![4,8,16].into_iter())))]
    #[django(op(in))]
    pub core_count: Option<u64>,
    #[boulder(default=Some("Example device type description.".to_string()))]
    #[django(op(in, contains, icontains, startswith, endswith))]
    pub description: Option<String>,
    #[boulder(default = 10)]
    #[django(op(in))]
    pub health_frequency: i64,
    #[boulder(default = false)]
    #[django(op(in))]
    pub disable_health_check: bool,
    #[boulder(default=HealthDenominator::Hours)]
    pub health_denominator: HealthDenominator,
    #[boulder(default = true)]
    #[django(op(in))]
    pub display: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HealthDenominator {
    Hours,
    Jobs,
}

impl django_query::filtering::ops::Scalar for HealthDenominator {}
impl django_query::row::StringCellValue for HealthDenominator {}
