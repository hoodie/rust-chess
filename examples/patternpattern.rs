enum FigureKind { Pawn, Bishop }

fn noo(a:u32, b:u32, c:u32) { println!("noo"); }
fn moo(a:u32, b:u32, c:u32) { println!("moo"); }
type MooNoo = fn(u32, u32, u32);

fn matchnoo(name:FigureKind) -> fn(u32,u32,u32)
{match name {
        FigureKind::Pawn    => noo,
        FigureKind::Bishop  => moo,
    }
}

fn main() {
    matchnoo(FigureKind::Pawn);
}
