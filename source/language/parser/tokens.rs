// Tagua VM
//
//
// New BSD License
//
// Copyright © 2016-2016, Ivan Enderlin.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the Hoa nor the names of its contributors may be
//       used to endorse or promote products derived from this software without
//       specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

//! List of lexemes.
//!
//! The list of all lexemes, aka tokens, is provided by the PHP Language
//! Specification in the [Grammar
//! chapter](https://github.com/php/php-langspec/blob/master/spec/19-grammar.md#lexical-grammar).
//!
//! All lexemes are declared as static string constants.

/// Helper to declare a token.
///
/// ### Examples
///
/// The following example declares the `FOO_BAR` token:
///
/// ```
/// token!(FOO_BAR: "foobar"; "The `FOO_BAR` token, mostly used in example.");
/// ```
macro_rules! token {
    ($name:ident: $value:expr; $documentation:expr) => (
        #[doc=$documentation]
        pub const $name: &'static str = $value;
    )
}

token!(ABSTRACT: "abstract"; "The `ABSTRACT` token.\n\nRepresent an abstract entity, e.g. `abstract class C { … }`.");
token!(AND: "and"; "The `AND` token.\n\nRepresent the conjunction operator, used in a logical expression, e.g. `$x and $y`.");
token!(ARRAY: "array"; "The `ARRAY` token.\n\nRepresent the array constructor, e.g. `array($x, $y)`.");
token!(AS: "as"; "The `AS` token.\n\nRepresent the alias operator, e.g. `use Foo\\Bar as Baz`.");
token!(BREAK: "break"; "The `BREAK` token.\n\nRepresent the control flow breaker operator, e.g. `break 2`.");
token!(CALLABLE: "callable"; "The `CALLABLE` token.\n\nRepresent the callable type, e.g. `function f(callable $x) { … }`.");
token!(CASE: "case"; "The `CASE` token.\n\nRepresent a case in a `switch` control structure, e.g. `switch (…) { case …: …; }`.");
token!(CATCH: "catch"; "The `CATCH` token.\n\nRepresent the `catch` block of a `try`/`catch` control structure, e.g. `try { … } catch (Exception $e) { … }`.");
token!(CLASS: "class"; "The `CLASS` token.\n\nRepresent the class declaration operator, e.g. `class C { … }`.");
token!(CLONE: "clone"; "The `CLONE` token.\n\nRepresent the clone operator, e.g. `clone $x`.");
token!(CONST: "const"; "The `CONST` token.\n\nRepresent the constant declaration operator, e.g. `const ANSWER = 42;`.");
token!(CONTINUE: "continue"; "The `CONTINUE` token.\n\nRepresent the control flow continuer operator, e.g. `continue 2;`.");
token!(DECLARE: "declare"; "The `DECLARE` token.\n\nRepresent the declaration operator, e.g. `declare(foo='bar');`.");
token!(DEFAULT: "default"; "The `DEFAULT` token.\n\nRepresent the default case in a `switch` control structure, e.g. `switch (…) { … default: …; }`.");
token!(DIE: "die"; "The `DIE` token.\n\nRepresent the termination operator, e.g. `die(42);`.");
token!(DO: "do"; "The `DO` token.\n\nRepresent the body of a `do`/`while` loop, e.g. `do { … } while (…);`.");
token!(DOLLARS: "$"; "The `DOLLARS` token.\n\nRepresent the introduction of a variable, e.g. `$x`.");
token!(ECHO: "echo"; "The `ECHO` token.\n\nRepresent the output writer operator, e.g. `echo 'foobar';`.");
token!(ELSE: "else"; "The `ELSE` token.\n\nRepresent the falsy block of a condition control structure, e.g. `if (…) { … } else { … }`.");
token!(ELSEIF: "elseif"; "The `ELSEIF` token.\n\nRepresent a `else if` block, e.g. `if (…) { … } elseif { … } else { … }`.");
token!(EMPTY: "empty"; "The `EMPTY` token.\n\nRepresent the emptiness operator, e.g. `empty($x)`.");
token!(ENDDECLARE: "enddeclare"; "The `ENDDECLARE` token.\n\nRepresent the end of a `declare` block, e.g. `declare: … enddeclare`.");
token!(ENDFOR: "endfor"; "The `ENDFOR` token.\n\nRepresent the end of a `for` block, e.g. `for (…; …; …): … endfor`.");
token!(ENDFOREACH: "endforeach"; "The `ENDFOREACH` token.\n\nRepresent the end of a `foreach` block, e.g. `foreach ($i as $k => $v): … endforeach`.");
token!(ENDIF: "endif"; "The `ENDIF` token.\n\nRepresent the end of an `if` block, e.g. `if (…): … endif`.");
token!(ENDSWITCH: "endswitch"; "The `ENDSWITCH` token.\n\nRepresent the end of a `switch` block, e.g. `switch(…): … endswitch`.");
token!(ENDWHILE: "endwhile"; "The `ENDWHILE` token.\n\nRepresent the end of a `while` block, e.g. `while(…): … endwhile`.");
token!(EQUAL: "="; "The `EQUAL` token.\n\nRepresent a binding of a value to a variable, e.g. `$x = 42`.");
token!(EVAL: "eval"; "The `EVAL` token.\n\nRepresent the late-evaluation operator, e.g. `eval($x)`.");
token!(EXIT: "exit"; "The `EXIT` token.\n\nRepresent the termination operator, e.g. `exit(42);`.");
token!(EXTENDS: "extends"; "The `EXTENDS` token.\n\nRepresent the inheritance operator, e.g. `class C extends D { … }`.");
token!(FINAL: "final"; "The `FINAL` token.\n\nRepresent a final entity, e.g. `final class C { … }`.");
token!(FINALLY: "finally"; "The `FINALLY` token.\n\nRepresent the finally block of a `try`/`catch` control structure, e.g. `try { … } catch (…) { … } finally { … }`.");
token!(FOR: "for"; "The `FOR` token.\n\nRepresent a `for` loop, e.g. `for (…; …; …) { … }`.");
token!(FOREACH: "foreach"; "The `FOREACH` token.\n\nRepresent a `foreach` loop, e.g. `foreach ($i as $k => $v) { … }`.");
token!(FUNCTION: "function"; "The `FUNCTION` token.\n\nRepresent the function declaration operator, e.g. `function f(…) { … }`.");
token!(GLOBAL: "global"; "The `GLOBAL` token.\n\nRepresent the global visibility modifier, e.g. `global $x`.");
token!(GOTO: "goto"; "The `GOTO` token.\n\nRepresent the jump operator, e.g. `goto x;`.");
token!(IF: "if"; "The `IF` token.\n\nRepresent the truly block of a condition control structure, e.g. `if (…) { … }`.");
token!(IMPLEMENTS: "implements"; "The `IMPLEMENTS` token.\n\nRepresent the implementation operator, e.g. `class C implements I { … }`.");
token!(INCLUDE: "include"; "The `INCLUDE` token.\n\nRepresent the import operator, e.g. `include $x;`.");
token!(INCLUDE_ONCE: "include_once"; "The `INCLUDE_ONCE` token.\n\nRepresent the import once operator, e.g. `include_once $x;`.");
token!(INSTANCEOF: "instanceof"; "The `INSTANCEOF` token.\n\nRepresent the subset operator, e.g. `$o instanceof C`.");
token!(INSTEADOF: "insteadof"; "The `INSTEADOF` token.\n\nRepresent the conflict resolution operator, `use C, D { C::f insteadof D }`.");
token!(INTERFACE: "interface"; "The `INTERFACE` token.\n\nRepresent the interface declaration operator, e.g. `interface I { … }`.");
token!(ISSET: "isset"; "The `ISSET` token.\n\nRepresent the existence operator, e.g. `isset($x)`.");
token!(LIST: "list"; "The `LIST` token.\n\nRepresent the destructuring operator, e.g. `list($x, $y) = $a`.");
token!(NAMESPACE: "namespace"; "The `NAMESPACE` token.\n\nRepresent the namespace declaration operator, e.g. `namespace N;`.");
token!(NEW: "new"; "The `NEW` token.\n\nRepresent the instanciation operator, e.g. `new C()`.");
token!(OR: "or"; "The `OR` token.\n\nRepresent the disjunction operator, used in a logical expression, e.g. `$x or $y`.");
token!(PLUS: "+"; "The `PLUS` token.\n\nRepresent the addition operator, used for instance in an arithmetic operation, e.g. `1 + 2`.");
token!(PRINT: "print"; "The `PRINT` token.\n\nRepresent another output writer operator, e.g. `print 'foobar';`, see `echo`.");
token!(PRIVATE: "private"; "The `PRIVATE` token.\n\nRepresent the private visibility operator, e.g. `private $x`.");
token!(PROTECTED: "protected"; "The `PROTECTED` token.\n\nRepresent the protected visibility operator, e.g. `protected $x`.");
token!(PUBLIC: "public"; "The `PUBLIC` token.\n\nRepresent the public visibility operator, e.g. `public $x`.");
token!(REQUIRE: "require"; "The `REQUIRE` token.\n\nRepresent the import operator, e.g. `require $x;`.");
token!(REQUIRE_ONCE: "require_once"; "The `REQUIRE_ONCE` token.\n\nRepresent the import once operator, e.g. `require_once $x;`.");
token!(RETURN: "return"; "The `RETURN` token.\n\nRepresent the return operator, e.g. `return $x;`.");
token!(SEMI_COLON: ";"; "The `SEMI_COLON` token.\n\nRepresent a terminator of an expression, e.g. `$x = 42;`.");
token!(STATIC: "static"; "The `STATIC` token.\n\nRepresent the stack declaration operator, e.g. `static $x`.");
token!(SWITCH: "switch"; "The `SWITCH` token.\n\nRepresent the switch control structure, e.g. `switch ($x) { … }`.");
token!(THROW: "throw"; "The `THROW` token.\n\nRepresent the throw exception operator, e.g. `throw $e;`.");
token!(TRAIT: "trait"; "The `TRAIT` token.\n\nRepresent the trait declaration operator, e.g. `trait T { … }`.");
token!(TRY: "try"; "The `TRY` token.\n\nRepresent the `try` block of a `try`/`catch` control structure, e.g. `try { … } catch (Exception $e) { … }`.");
token!(UNSET: "unset"; "The `UNSET` token.\n\nRepresent the destruction operator, e.g. `unset($x);`.");
token!(USE: "use"; "The `USE` token.\n\nRepresent the importing operator (for namespaces or traits for instance), e.g. `use C\\D;`.");
token!(VAR: "var"; "The `VAR` token.\n\nRepresent the variable declaration operator (for old PHP versions), e.g. `var $x = …;`.");
token!(WHILE: "while"; "The `WHILE` token.\n\nRepresent a `while` loop, e.g. `while (…) { … }`.");
token!(XOR: "xor"; "The `XOR` token.\n\nRepresent the exclusive disjunction operator, used in a logical expression, e.g. `$x xor $y`.");
token!(YIELD: "yield"; "The `YIELD` token.\n\nRepresent the generator operator, e.g. `yield …;`.");
token!(YIELD_FROM: "yield from"; "The `YIELD_FROM` token.\n\nRepresent the delegated generator operator, e.g. `yield from …;`.");
