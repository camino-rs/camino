(function() {
    var type_impls = Object.fromEntries([["proptest",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Arbitrary-for-(T0,+T1)\" class=\"impl\"><a class=\"src rightside\" href=\"src/proptest/arbitrary/tuples.rs.html#30\">Source</a><a href=\"#impl-Arbitrary-for-(T0,+T1)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T0: <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a>, T1: <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a>&gt; <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.tuple.html\">(T0, T1)</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Parameters\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/proptest/arbitrary/tuples.rs.html#30\">Source</a><a href=\"#associatedtype.Parameters\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Parameters\" class=\"associatedtype\">Parameters</a> = (&lt;T0 as <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a>&gt;::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Parameters\" title=\"type proptest::arbitrary::Arbitrary::Parameters\">Parameters</a>, &lt;T1 as <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a>&gt;::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Parameters\" title=\"type proptest::arbitrary::Arbitrary::Parameters\">Parameters</a>)</h4></section></summary><div class='docblock'>The type of parameters that <a href=\"trait.Arbitrary.html#tymethod.arbitrary_with\"><code>arbitrary_with</code></a> accepts for configuration\nof the generated <a href=\"../strategy/trait.Strategy.html\"><code>Strategy</code></a>. Parameters must implement <a href=\"https://doc.rust-lang.org/nightly/std/default/trait.Default.html\"><code>Default</code></a>.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Strategy\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/proptest/arbitrary/tuples.rs.html#30\">Source</a><a href=\"#associatedtype.Strategy\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Strategy\" class=\"associatedtype\">Strategy</a> = (&lt;T0 as <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a>&gt;::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Strategy\" title=\"type proptest::arbitrary::Arbitrary::Strategy\">Strategy</a>, &lt;T1 as <a class=\"trait\" href=\"proptest/arbitrary/trait.Arbitrary.html\" title=\"trait proptest::arbitrary::Arbitrary\">Arbitrary</a>&gt;::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Strategy\" title=\"type proptest::arbitrary::Arbitrary::Strategy\">Strategy</a>)</h4></section></summary><div class='docblock'>The type of <a href=\"../strategy/trait.Strategy.html\"><code>Strategy</code></a> used to generate values of type <code>Self</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.arbitrary_with\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/arbitrary/tuples.rs.html#30\">Source</a><a href=\"#method.arbitrary_with\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/arbitrary/trait.Arbitrary.html#tymethod.arbitrary_with\" class=\"fn\">arbitrary_with</a>(args: Self::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Parameters\" title=\"type proptest::arbitrary::Arbitrary::Parameters\">Parameters</a>) -&gt; Self::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Strategy\" title=\"type proptest::arbitrary::Arbitrary::Strategy\">Strategy</a></h4></section></summary><div class='docblock'>Generates a <a href=\"../strategy/trait.Strategy.html\"><code>Strategy</code></a> for producing arbitrary values of type the\nimplementing type (<code>Self</code>). The strategy is passed the arguments given\nin args. <a href=\"proptest/arbitrary/trait.Arbitrary.html#tymethod.arbitrary_with\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.arbitrary\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/arbitrary/traits.rs.html#74-76\">Source</a><a href=\"#method.arbitrary\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/arbitrary/trait.Arbitrary.html#method.arbitrary\" class=\"fn\">arbitrary</a>() -&gt; Self::<a class=\"associatedtype\" href=\"proptest/arbitrary/trait.Arbitrary.html#associatedtype.Strategy\" title=\"type proptest::arbitrary::Arbitrary::Strategy\">Strategy</a></h4></section></summary><div class='docblock'>Generates a <a href=\"../strategy/trait.Strategy.html\"><code>Strategy</code></a> for producing arbitrary values\nof type the implementing type (<code>Self</code>). <a href=\"proptest/arbitrary/trait.Arbitrary.html#method.arbitrary\">Read more</a></div></details></div></details>","Arbitrary","proptest::strategy::unions::W","proptest::strategy::unions::WA"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Strategy-for-(A,+B)\" class=\"impl\"><a class=\"src rightside\" href=\"src/proptest/tuple.rs.html#93\">Source</a><a href=\"#impl-Strategy-for-(A,+B)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;A: <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>, B: <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>&gt; <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.tuple.html\">(A, B)</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Tree\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/proptest/tuple.rs.html#93\">Source</a><a href=\"#associatedtype.Tree\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"proptest/strategy/trait.Strategy.html#associatedtype.Tree\" class=\"associatedtype\">Tree</a> = <a class=\"struct\" href=\"proptest/tuple/struct.TupleValueTree.html\" title=\"struct proptest::tuple::TupleValueTree\">TupleValueTree</a>&lt;(&lt;A as <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>&gt;::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Tree\" title=\"type proptest::strategy::Strategy::Tree\">Tree</a>, &lt;B as <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>&gt;::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Tree\" title=\"type proptest::strategy::Strategy::Tree\">Tree</a>)&gt;</h4></section></summary><div class='docblock'>The value tree generated by this <code>Strategy</code>.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Value\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/proptest/tuple.rs.html#93\">Source</a><a href=\"#associatedtype.Value\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" class=\"associatedtype\">Value</a> = (&lt;A as <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>&gt;::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>, &lt;B as <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>&gt;::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>)</h4></section></summary><div class='docblock'>The type of value used by functions under test generated by this Strategy. <a href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_tree\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/tuple.rs.html#93\">Source</a><a href=\"#method.new_tree\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#tymethod.new_tree\" class=\"fn\">new_tree</a>(&amp;self, runner: &amp;mut <a class=\"struct\" href=\"proptest/test_runner/struct.TestRunner.html\" title=\"struct proptest::test_runner::TestRunner\">TestRunner</a>) -&gt; <a class=\"type\" href=\"proptest/strategy/type.NewTree.html\" title=\"type proptest::strategy::NewTree\">NewTree</a>&lt;Self&gt;</h4></section></summary><div class='docblock'>Generate a new value tree from the given runner. <a href=\"proptest/strategy/trait.Strategy.html#tymethod.new_tree\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#74-85\">Source</a><a href=\"#method.prop_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_map\" class=\"fn\">prop_map</a>&lt;O: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>) -&gt; O&gt;(self, fun: F) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.Map.html\" title=\"struct proptest::strategy::Map\">Map</a>&lt;Self, F&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Returns a strategy which produces values transformed by the function\n<code>fun</code>. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_map_into\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#96-102\">Source</a><a href=\"#method.prop_map_into\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_map_into\" class=\"fn\">prop_map_into</a>&lt;O: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt;(self) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.MapInto.html\" title=\"struct proptest::strategy::MapInto\">MapInto</a>&lt;Self, O&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;O&gt;,</div></h4></section></summary><div class='docblock'>Returns a strategy which produces values of type <code>O</code> by transforming\n<code>Self</code> with <code>Into&lt;O&gt;</code>. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_map_into\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_perturb\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#137-148\">Source</a><a href=\"#method.prop_perturb\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_perturb\" class=\"fn\">prop_perturb</a>&lt;O: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>, <a class=\"struct\" href=\"proptest/test_runner/struct.TestRng.html\" title=\"struct proptest::test_runner::TestRng\">TestRng</a>) -&gt; O&gt;(\n    self,\n    fun: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.Perturb.html\" title=\"struct proptest::strategy::Perturb\">Perturb</a>&lt;Self, F&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Returns a strategy which produces values transformed by the function\n<code>fun</code>, which is additionally given a random number generator. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_perturb\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_flat_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#238-249\">Source</a><a href=\"#method.prop_flat_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_flat_map\" class=\"fn\">prop_flat_map</a>&lt;S: <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>) -&gt; S&gt;(\n    self,\n    fun: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.Flatten.html\" title=\"struct proptest::strategy::Flatten\">Flatten</a>&lt;<a class=\"struct\" href=\"proptest/strategy/struct.Map.html\" title=\"struct proptest::strategy::Map\">Map</a>&lt;Self, F&gt;&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Maps values produced by this strategy into new strategies and picks\nvalues from those strategies. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_flat_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_ind_flat_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#264-275\">Source</a><a href=\"#method.prop_ind_flat_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_ind_flat_map\" class=\"fn\">prop_ind_flat_map</a>&lt;S: <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>) -&gt; S&gt;(\n    self,\n    fun: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.IndFlatten.html\" title=\"struct proptest::strategy::IndFlatten\">IndFlatten</a>&lt;<a class=\"struct\" href=\"proptest/strategy/struct.Map.html\" title=\"struct proptest::strategy::Map\">Map</a>&lt;Self, F&gt;&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Maps values produced by this strategy into new strategies and picks\nvalues from those strategies while considering the new strategies to be\nindependent. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_ind_flat_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_ind_flat_map2\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#282-293\">Source</a><a href=\"#method.prop_ind_flat_map2\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_ind_flat_map2\" class=\"fn\">prop_ind_flat_map2</a>&lt;S: <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>) -&gt; S&gt;(\n    self,\n    fun: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.IndFlattenMap.html\" title=\"struct proptest::strategy::IndFlattenMap\">IndFlattenMap</a>&lt;Self, F&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Similar to <code>prop_ind_flat_map()</code>, but produces 2-tuples with the input\ngenerated from <code>self</code> in slot 0 and the derived strategy in slot 1. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_ind_flat_map2\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_filter\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#315-324\">Source</a><a href=\"#method.prop_filter\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_filter\" class=\"fn\">prop_filter</a>&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"proptest/test_runner/struct.Reason.html\" title=\"struct proptest::test_runner::Reason\">Reason</a>&gt;, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(&amp;Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.bool.html\">bool</a>&gt;(\n    self,\n    whence: R,\n    fun: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.Filter.html\" title=\"struct proptest::strategy::Filter\">Filter</a>&lt;Self, F&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Returns a strategy which only produces values accepted by <code>fun</code>. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_filter\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_filter_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#349-358\">Source</a><a href=\"#method.prop_filter_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_filter_map\" class=\"fn\">prop_filter_map</a>&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;O&gt;, O: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt;(\n    self,\n    whence: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"proptest/test_runner/struct.Reason.html\" title=\"struct proptest::test_runner::Reason\">Reason</a>&gt;,\n    fun: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.FilterMap.html\" title=\"struct proptest::strategy::FilterMap\">FilterMap</a>&lt;Self, F&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Returns a strategy which only produces transformed values where <code>fun</code>\nreturns <code>Some(value)</code> and rejects those where <code>fun</code> returns <code>None</code>. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_filter_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_union\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#374-379\">Source</a><a href=\"#method.prop_union\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_union\" class=\"fn\">prop_union</a>(self, other: Self) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.Union.html\" title=\"struct proptest::strategy::Union\">Union</a>&lt;Self&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Returns a strategy which picks uniformly from <code>self</code> and <code>other</code>. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_union\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prop_recursive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#448-462\">Source</a><a href=\"#method.prop_recursive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.prop_recursive\" class=\"fn\">prop_recursive</a>&lt;R: <a class=\"trait\" href=\"proptest/strategy/trait.Strategy.html\" title=\"trait proptest::strategy::Strategy\">Strategy</a>&lt;Value = Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>&gt; + 'static, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"struct\" href=\"proptest/strategy/struct.BoxedStrategy.html\" title=\"struct proptest::strategy::BoxedStrategy\">BoxedStrategy</a>&lt;Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>&gt;) -&gt; R&gt;(\n    self,\n    depth: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u32.html\">u32</a>,\n    desired_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u32.html\">u32</a>,\n    expected_branch_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u32.html\">u32</a>,\n    recurse: F,\n) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.Recursive.html\" title=\"struct proptest::strategy::Recursive\">Recursive</a>&lt;Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>, F&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'static,</div></h4></section></summary><div class='docblock'>Generate a recursive structure with <code>self</code> items as leaves. <a href=\"proptest/strategy/trait.Strategy.html#method.prop_recursive\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.boxed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#516-521\">Source</a><a href=\"#method.boxed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.boxed\" class=\"fn\">boxed</a>(self) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.BoxedStrategy.html\" title=\"struct proptest::strategy::BoxedStrategy\">BoxedStrategy</a>&lt;Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'static,</div></h4></section></summary><div class='docblock'>Erases the type of this <code>Strategy</code> so it can be passed around as a\nsimple trait object. <a href=\"proptest/strategy/trait.Strategy.html#method.boxed\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.sboxed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#531-536\">Source</a><a href=\"#method.sboxed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.sboxed\" class=\"fn\">sboxed</a>(self) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.SBoxedStrategy.html\" title=\"struct proptest::strategy::SBoxedStrategy\">SBoxedStrategy</a>&lt;Self::<a class=\"associatedtype\" href=\"proptest/strategy/trait.Strategy.html#associatedtype.Value\" title=\"type proptest::strategy::Strategy::Value\">Value</a>&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class='docblock'>Erases the type of this <code>Strategy</code> so it can be passed around as a\nsimple trait object. <a href=\"proptest/strategy/trait.Strategy.html#method.sboxed\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.no_shrink\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/proptest/strategy/traits.rs.html#547-552\">Source</a><a href=\"#method.no_shrink\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"proptest/strategy/trait.Strategy.html#method.no_shrink\" class=\"fn\">no_shrink</a>(self) -&gt; <a class=\"struct\" href=\"proptest/strategy/struct.NoShrink.html\" title=\"struct proptest::strategy::NoShrink\">NoShrink</a>&lt;Self&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Wraps this strategy to prevent values from being subject to shrinking. <a href=\"proptest/strategy/trait.Strategy.html#method.no_shrink\">Read more</a></div></details></div></details>","Strategy","proptest::strategy::unions::W","proptest::strategy::unions::WA"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[29493]}