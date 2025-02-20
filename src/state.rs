pub trait ResultTranform<AMario, BMario>
where
    Self: Transformer,
    AMario: Transformee,
    BMario: Transformee,
{
}

pub trait ManualTransform<ThisMario, ThatMario>
where
    Self: Transformer,
    ThisMario: Transformee,
    ThatMario: Transformee,
{
}

pub trait IdemTransform<ThisMario, ThatMario>
where
    Self: Transformer,
    ThisMario: Transformee,
    ThatMario: Transformee,
{
    type FromMario: Transformee;
}

pub trait SkipTransform<ThisMario, ThatMario>
where
    Self: Transformer,
    ThisMario: Transformee,
    ThatMario: Transformee,
{
    type Skip: Transformee;
}

trait Transformee {}
trait Transformer {}
trait Idem {}

macro_rules! make {
    (tranformee $name:ident) => {
        #[derive(Default)]
        pub struct $name;
        impl Transformee for $name {}
    };
    (tranformer $name:ident) => {
        pub struct $name;
        impl Transformer for $name {}
    };
}

macro_rules! rule {
    ($tranformee:ident + $tranformer:ident => $transformed:ident) => {
        impl ManualTransform<$tranformee, $transformed> for $tranformer {}
    };
    ($tranformee:ident + $tranformer:ident => $transformed:ident idem) => {
        impl ManualTransform<$tranformee, $transformed> for $tranformer {}
        impl Idem for $tranformer {}
    };
}

make!(tranformee DeadMario);
make!(tranformee SmallMario);
make!(tranformee NormalMario);
make!(tranformee FireMario);
make!(tranformee CapeMario);

make!(tranformer FireFlower);
make!(tranformer FlyFlower);
make!(tranformer Damage);

rule!(NormalMario + FireFlower => FireMario idem);
rule!(NormalMario + FlyFlower => CapeMario idem);
rule!(FireMario + Damage => NormalMario);
rule!(CapeMario + Damage => NormalMario);
rule!(NormalMario + Damage => SmallMario);
rule!(SmallMario + Damage => DeadMario);

// for every idem-ponent transformation of A -> B with T,
// an idem-potent transformation of B -> B with T is crated
impl<Flower, AMario, BMario> IdemTransform<BMario, BMario> for Flower
where
    Flower: ManualTransform<AMario, BMario> + Idem,
    AMario: Transformee,
    BMario: Transformee,
{
    type FromMario = AMario;
}

// for every idem-potent tranformer that acts on NormalMario,
// that same tranformer can act on SmallMario and generate the same Transformee
impl<Flower, SuperMario: Transformee> SkipTransform<SmallMario, SuperMario> for Flower
where
    Flower: ManualTransform<NormalMario, SuperMario> + Idem,
{
    type Skip = NormalMario;
}

impl<AMario, BMario, Flower> ResultTranform<AMario, BMario> for Flower
where
    Flower: SkipTransform<AMario, BMario>,
    AMario: Transformee,
    BMario: Transformee,
{
}

impl<Flower, AMario, BMario> ResultTranform<AMario, BMario> for Flower
where
    Flower: IdemTransform<AMario, BMario>,
    AMario: Transformee,
    BMario: Transformee

{
}
