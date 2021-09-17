// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use crate::json_grammar::JsonGrammar;
use id_tree::Tree;
use parol_runtime::parser::errors::*;
use parol_runtime::parser::{AstStackEntry, AstType, UserActionsTrait};

///
/// The `JsonGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait JsonGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// Json: Value;
    ///
    fn json_0(&mut self, _value_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// Object: "\{" ObjectSuffix1;
    ///
    fn object_1(
        &mut self,
        _l_brace_0: &AstStackEntry,
        _object_suffix1_1: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ObjectSuffix1: Pair ObjectSuffix;
    ///
    fn object_suffix1_2(
        &mut self,
        _pair_0: &AstStackEntry,
        _object_suffix_1: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ObjectSuffix1: "\}";
    ///
    fn object_suffix1_3(
        &mut self,
        _r_brace_0: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ObjectSuffix: ObjectRest "\}";
    ///
    fn object_suffix_4(
        &mut self,
        _object_rest_0: &AstStackEntry,
        _r_brace_1: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// ObjectSuffix: "\}";
    ///
    fn object_suffix_5(
        &mut self,
        _r_brace_0: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// ObjectRest: "," Pair ObjectRestSuffix;
    ///
    fn object_rest_6(
        &mut self,
        _comma_0: &AstStackEntry,
        _pair_1: &AstStackEntry,
        _object_rest_suffix_2: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// ObjectRestSuffix: ObjectRest;
    ///
    fn object_rest_suffix_7(
        &mut self,
        _object_rest_0: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// ObjectRestSuffix: ;
    ///
    fn object_rest_suffix_8(&mut self, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// Pair: String ":" Value;
    ///
    fn pair_9(
        &mut self,
        _string_0: &AstStackEntry,
        _colon_1: &AstStackEntry,
        _value_2: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// Array: "\[" ArraySuffix1;
    ///
    fn array_10(
        &mut self,
        _l_bracket_0: &AstStackEntry,
        _array_suffix1_1: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// ArraySuffix1: Value ArraySuffix;
    ///
    fn array_suffix1_11(
        &mut self,
        _value_0: &AstStackEntry,
        _array_suffix_1: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// ArraySuffix1: "\]";
    ///
    fn array_suffix1_12(
        &mut self,
        _r_bracket_0: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// ArraySuffix: ArrayRest "\]";
    ///
    fn array_suffix_13(
        &mut self,
        _array_rest_0: &AstStackEntry,
        _r_bracket_1: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// ArraySuffix: "\]";
    ///
    fn array_suffix_14(
        &mut self,
        _r_bracket_0: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// ArrayRest: "," Value ArrayRestSuffix;
    ///
    fn array_rest_15(
        &mut self,
        _comma_0: &AstStackEntry,
        _value_1: &AstStackEntry,
        _array_rest_suffix_2: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// ArrayRestSuffix: ArrayRest;
    ///
    fn array_rest_suffix_16(
        &mut self,
        _array_rest_0: &AstStackEntry,
        _parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// ArrayRestSuffix: ;
    ///
    fn array_rest_suffix_17(&mut self, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// Value: String;
    ///
    fn value_18(&mut self, _string_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// Value: Number;
    ///
    fn value_19(&mut self, _number_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// Value: Object;
    ///
    fn value_20(&mut self, _object_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 21:
    ///
    /// Value: Array;
    ///
    fn value_21(&mut self, _array_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// Value: "true";
    ///
    fn value_22(&mut self, _true_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// Value: "false";
    ///
    fn value_23(&mut self, _false_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// Value: "null";
    ///
    fn value_24(&mut self, _null_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// String: "\u{0022}(\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    ///
    fn string_25(&mut self, _string_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// Number: "-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][-+]?(0|[1-9][0-9]*)?)?";
    ///
    fn number_26(&mut self, _number_0: &AstStackEntry, _parse_tree: &Tree<AstType>) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait for JsonGrammar {
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[AstStackEntry],
        parse_tree: &Tree<AstType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.json_0(&children[0], parse_tree),

            1 => self.object_1(&children[0], &children[1], parse_tree),

            2 => self.object_suffix1_2(&children[0], &children[1], parse_tree),

            3 => self.object_suffix1_3(&children[0], parse_tree),

            4 => self.object_suffix_4(&children[0], &children[1], parse_tree),

            5 => self.object_suffix_5(&children[0], parse_tree),

            6 => self.object_rest_6(&children[0], &children[1], &children[2], parse_tree),

            7 => self.object_rest_suffix_7(&children[0], parse_tree),

            8 => self.object_rest_suffix_8(parse_tree),

            9 => self.pair_9(&children[0], &children[1], &children[2], parse_tree),

            10 => self.array_10(&children[0], &children[1], parse_tree),

            11 => self.array_suffix1_11(&children[0], &children[1], parse_tree),

            12 => self.array_suffix1_12(&children[0], parse_tree),

            13 => self.array_suffix_13(&children[0], &children[1], parse_tree),

            14 => self.array_suffix_14(&children[0], parse_tree),

            15 => self.array_rest_15(&children[0], &children[1], &children[2], parse_tree),

            16 => self.array_rest_suffix_16(&children[0], parse_tree),

            17 => self.array_rest_suffix_17(parse_tree),

            18 => self.value_18(&children[0], parse_tree),

            19 => self.value_19(&children[0], parse_tree),

            20 => self.value_20(&children[0], parse_tree),

            21 => self.value_21(&children[0], parse_tree),

            22 => self.value_22(&children[0], parse_tree),

            23 => self.value_23(&children[0], parse_tree),

            24 => self.value_24(&children[0], parse_tree),

            25 => self.string_25(&children[0], parse_tree),

            26 => self.number_26(&children[0], parse_tree),

            _ => panic!("Unhandled production number: {}", prod_num),
        }
    }
}
