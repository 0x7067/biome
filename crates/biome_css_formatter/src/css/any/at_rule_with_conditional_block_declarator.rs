//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRuleWithConditionalBlockDeclarator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRuleWithConditionalBlockDeclarator;
impl FormatRule<AnyCssAtRuleWithConditionalBlockDeclarator>
    for FormatAnyCssAtRuleWithConditionalBlockDeclarator
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssAtRuleWithConditionalBlockDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssAtRuleWithConditionalBlockDeclarator::CssContainerAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithConditionalBlockDeclarator::CssMediaAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithConditionalBlockDeclarator::CssScopeAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithConditionalBlockDeclarator::CssStartingStyleAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithConditionalBlockDeclarator::CssSupportsAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
        }
    }
}
