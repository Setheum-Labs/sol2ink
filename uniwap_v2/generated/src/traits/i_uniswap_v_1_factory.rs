// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type IUniswapV1FactoryRef = dyn IUniswapV1Factory;

#[openbrush::trait_definition]
pub trait IUniswapV1Factory {
    #[ink(message)]
    fn get_exchange(&self, _: AccountId) -> Result<AccountId, Error>;

}
