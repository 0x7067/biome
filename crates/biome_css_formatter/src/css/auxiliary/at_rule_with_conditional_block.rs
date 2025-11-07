use crate::prelude::*;
use biome_css_syntax::{CssAtRuleWithConditionalBlock, CssAtRuleWithConditionalBlockFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAtRuleWithConditionalBlock;

impl FormatNodeRule<CssAtRuleWithConditionalBlock> for FormatCssAtRuleWithConditionalBlock {
    fn fmt_fields(
        &self,
        node: &CssAtRuleWithConditionalBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssAtRuleWithConditionalBlockFields { declarator, block } = node.as_fields();

        write!(f, [declarator.format(), space(), block.format()])
    }
}
