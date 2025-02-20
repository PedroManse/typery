trait Transform<ThisMario>
where
    Self: Transformer,
    ThisMario: Transformee,
{
    type To: Default;
}

trait Transformee {}
trait Transformer {}
trait Idem {}

macro_rules! make {
    (tranformee $name:ident) => {
        #[derive(Default)]
        struct $name;
        impl Transformee for $name {}
    };
    (tranformer $name:ident) => {
        impl Transformer for $name {}
        struct $name;
    }
}

macro_rules! rule {
    ($tranformee:ident + $tranformer:ident => $transformed:ident) => {
        impl Transform<$tranformee> for $tranformer { type To = $transformed; }
    };
    ($tranformee:ident + $tranformer:ident => $transformed:ident idem) => {
        impl Transform<$tranformee> for $tranformer { type To = $transformed; }
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

// for every idem-ponent transformation of a -t> b, create an idem-potent transformation of b -t> b
///impl<T: Transformer, Mf: Transformee, Mr: Transformee> Transform<Mr> for T
///where T: Transform<Mf> + Idem
///{
///    type To = Mf;
///}

// for every idem-potent tranformer that acts on NormalMario, replicate it's effect on SmallMario
impl<T: Transformer> Transform<SmallMario> for T
where T: Transform<NormalMario> + Idem
{
    type To = <T as Transform<NormalMario>>::To;
}
