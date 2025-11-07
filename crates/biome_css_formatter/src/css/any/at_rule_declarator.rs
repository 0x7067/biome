//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRuleDeclarator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRuleDeclarator;
impl FormatRule<AnyCssAtRuleDeclarator> for FormatAnyCssAtRuleDeclarator {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAtRuleDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAtRuleDeclarator::AnyCssAtRuleWithConditionalBlockDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleDeclarator::AnyCssAtRuleWithDeclarationBlockDeclarator(node) => {
                node.format().fmt(f)
            }
        }
    }
}
