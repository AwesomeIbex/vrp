use crate::construction::states::InsertionContext;

/// Specifies ruin strategy.
pub trait Ruin {
    fn run(&self, mut insertion_ctx: InsertionContext) -> InsertionContext;
}

mod adjusted_string_removal;
pub use self::adjusted_string_removal::AdjustedStringRemoval;

mod random_route_removal;
pub use self::random_route_removal::RandomRouteRemoval;

/// Provides the way to run multiple ruin methods.
pub struct CompositeRuin {
    ruins: Vec<(Box<dyn Ruin>, f64)>,
}

impl Default for CompositeRuin {
    fn default() -> Self {
        Self {
            ruins: vec![
                (Box::new(AdjustedStringRemoval::default()), 1.),
                (Box::new(RandomRouteRemoval::default()), 0.01),
            ],
        }
    }
}

impl Ruin for CompositeRuin {
    fn run(&self, mut insertion_ctx: InsertionContext) -> InsertionContext {
        let random = insertion_ctx.random.clone();
        self.ruins
            .iter()
            .filter(|(_, probability)| *probability > random.uniform_real(0., 1.))
            .fold(insertion_ctx, |mut ctx, (ruin, _)| ruin.run(ctx))
    }
}