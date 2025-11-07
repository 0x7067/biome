use crate::prelude::*;
use biome_css_syntax::{CssAtRuleWithDeclarationBlock, CssAtRuleWithDeclarationBlockFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAtRuleWithDeclarationBlock;

impl FormatNodeRule<CssAtRuleWithDeclarationBlock> for FormatCssAtRuleWithDeclarationBlock {
    fn fmt_fields(
        &self,
        node: &CssAtRuleWithDeclarationBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssAtRuleWithDeclarationBlockFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
