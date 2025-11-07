//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRuleWithDeclarationBlockDeclarator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRuleWithDeclarationBlockDeclarator;
impl FormatRule<AnyCssAtRuleWithDeclarationBlockDeclarator>
    for FormatAnyCssAtRuleWithDeclarationBlockDeclarator
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssAtRuleWithDeclarationBlockDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssColorProfileAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssCounterStyleAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssFontFaceAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssFontPaletteValuesAtRuleDeclarator(
                node,
            ) => node.format().fmt(f),
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssPositionTryAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssPropertyAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleWithDeclarationBlockDeclarator::CssViewTransitionAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
        }
    }
}
