use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::{AstNode, declare_node_union};

declare_lint_rule! {
    /// Enforce initial values when initializing the Svelte stores.
    ///
    /// Not initializing a Svelte store can make code ambiguous or difficult to understand.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const w1 = writable();
    /// export const r1 = readable();
    /// export const d1 = derived([a, b], () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const w1 = writable(false);
    /// export const r1 = readable({});
    /// export const d1 = derived([a, b], () => {}, false);
    /// ```
    ///
    pub LintSvelteRequireStoresInit {
        version: "next",
        name: "lintSvelteRequireStoresInit",
        language: "js",
        sources: &[RuleSource::EslintSvelte("require-stores-init")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

declare_node_union! {
    pub AnySvelteStore = JsForStatement | JsForInStatement | JsForOfStatement | JsWhileStatement | JsDoWhileStatement | JsWithStatement
}

impl Rule for LintSvelteRequireStoresInit {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
