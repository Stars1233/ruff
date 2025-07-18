use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, CmpOp, Expr};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;
use crate::registry::Rule;

use crate::rules::flake8_2020::helpers::is_sys;

/// ## What it does
/// Checks for comparisons that test `sys.version` against string literals,
/// such that the comparison will evaluate to `False` on Python 3.10 or later.
///
/// ## Why is this bad?
/// Comparing `sys.version` to a string is error-prone and may cause subtle
/// bugs, as the comparison will be performed lexicographically, not
/// semantically. For example, `sys.version > "3.9"` will evaluate to `False`
/// when using Python 3.10, as `"3.10"` is lexicographically "less" than
/// `"3.9"`.
///
/// Instead, use `sys.version_info` to access the current major and minor
/// version numbers as a tuple, which can be compared to other tuples
/// without issue.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.version > "3.9"  # `False` on Python 3.10.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// sys.version_info > (3, 9)  # `True` on Python 3.10.
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub(crate) struct SysVersionCmpStr3;

impl Violation for SysVersionCmpStr3 {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version` compared to string (python3.10), use `sys.version_info`".to_string()
    }
}

/// ## What it does
/// Checks for equality comparisons against the major version returned by
/// `sys.version_info` (e.g., `sys.version_info[0] == 3` or `sys.version_info[0] != 3`).
///
/// ## Why is this bad?
/// Using `sys.version_info[0] == 3` to verify that the major version is
/// Python 3 or greater will fail if the major version number is ever
/// incremented (e.g., to Python 4). This is likely unintended, as code
/// that uses this comparison is likely intended to be run on Python 2,
/// but would now run on Python 4 too. Similarly, using `sys.version_info[0] != 3`
/// to check for Python 2 will also fail if the major version number is
/// incremented.
///
/// Instead, use `>=` to check if the major version number is 3 or greater,
/// or `<` to check if the major version number is less than 3, to future-proof
/// the code.
///
/// ## Example
/// ```python
/// import sys
///
/// if sys.version_info[0] == 3:
///     ...
/// else:
///     print("Python 2")  # This will be printed on Python 4.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// if sys.version_info >= (3,):
///     ...
/// else:
///     print("Python 2")  # This will not be printed on Python 4.
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub(crate) struct SysVersionInfo0Eq3 {
    eq: bool,
}

impl Violation for SysVersionInfo0Eq3 {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.eq {
            "`sys.version_info[0] == 3` referenced (python4), use `>=`".to_string()
        } else {
            "`sys.version_info[0] != 3` referenced (python4), use `<`".to_string()
        }
    }
}

/// ## What it does
/// Checks for comparisons that test `sys.version_info[1]` against an integer.
///
/// ## Why is this bad?
/// Comparisons based on the current minor version number alone can cause
/// subtle bugs and would likely lead to unintended effects if the Python
/// major version number were ever incremented (e.g., to Python 4).
///
/// Instead, compare `sys.version_info` to a tuple, including the major and
/// minor version numbers, to future-proof the code.
///
/// ## Example
/// ```python
/// import sys
///
/// if sys.version_info[1] < 7:
///     print("Python 3.6 or earlier.")  # This will be printed on Python 4.0.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// if sys.version_info < (3, 7):
///     print("Python 3.6 or earlier.")
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub(crate) struct SysVersionInfo1CmpInt;

impl Violation for SysVersionInfo1CmpInt {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version_info[1]` compared to integer (python4), compare `sys.version_info` to \
             tuple"
            .to_string()
    }
}

/// ## What it does
/// Checks for comparisons that test `sys.version_info.minor` against an integer.
///
/// ## Why is this bad?
/// Comparisons based on the current minor version number alone can cause
/// subtle bugs and would likely lead to unintended effects if the Python
/// major version number were ever incremented (e.g., to Python 4).
///
/// Instead, compare `sys.version_info` to a tuple, including the major and
/// minor version numbers, to future-proof the code.
///
/// ## Example
/// ```python
/// import sys
///
/// if sys.version_info.minor < 7:
///     print("Python 3.6 or earlier.")  # This will be printed on Python 4.0.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// if sys.version_info < (3, 7):
///     print("Python 3.6 or earlier.")
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub(crate) struct SysVersionInfoMinorCmpInt;

impl Violation for SysVersionInfoMinorCmpInt {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version_info.minor` compared to integer (python4), compare `sys.version_info` \
             to tuple"
            .to_string()
    }
}

/// ## What it does
/// Checks for comparisons that test `sys.version` against string literals,
/// such that the comparison would fail if the major version number were
/// ever incremented to Python 10 or higher.
///
/// ## Why is this bad?
/// Comparing `sys.version` to a string is error-prone and may cause subtle
/// bugs, as the comparison will be performed lexicographically, not
/// semantically.
///
/// Instead, use `sys.version_info` to access the current major and minor
/// version numbers as a tuple, which can be compared to other tuples
/// without issue.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.version >= "3"  # `False` on Python 10.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// sys.version_info >= (3,)  # `True` on Python 10.
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub(crate) struct SysVersionCmpStr10;

impl Violation for SysVersionCmpStr10 {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version` compared to string (python10), use `sys.version_info`".to_string()
    }
}

/// YTT103, YTT201, YTT203, YTT204, YTT302
pub(crate) fn compare(checker: &Checker, left: &Expr, ops: &[CmpOp], comparators: &[Expr]) {
    match left {
        Expr::Subscript(ast::ExprSubscript { value, slice, .. })
            if is_sys(value, "version_info", checker.semantic()) =>
        {
            if let Expr::NumberLiteral(ast::ExprNumberLiteral {
                value: ast::Number::Int(i),
                ..
            }) = slice.as_ref()
            {
                if *i == 0 {
                    if let (
                        [operator @ (CmpOp::Eq | CmpOp::NotEq)],
                        [
                            Expr::NumberLiteral(ast::ExprNumberLiteral {
                                value: ast::Number::Int(n),
                                ..
                            }),
                        ],
                    ) = (ops, comparators)
                    {
                        if *n == 3 && checker.is_rule_enabled(Rule::SysVersionInfo0Eq3) {
                            checker.report_diagnostic(
                                SysVersionInfo0Eq3 {
                                    eq: matches!(*operator, CmpOp::Eq),
                                },
                                left.range(),
                            );
                        }
                    }
                } else if *i == 1 {
                    if let (
                        [CmpOp::Lt | CmpOp::LtE | CmpOp::Gt | CmpOp::GtE],
                        [
                            Expr::NumberLiteral(ast::ExprNumberLiteral {
                                value: ast::Number::Int(_),
                                ..
                            }),
                        ],
                    ) = (ops, comparators)
                    {
                        checker.report_diagnostic_if_enabled(SysVersionInfo1CmpInt, left.range());
                    }
                }
            }
        }

        Expr::Attribute(ast::ExprAttribute { value, attr, .. })
            if is_sys(value, "version_info", checker.semantic()) && attr == "minor" =>
        {
            if let (
                [CmpOp::Lt | CmpOp::LtE | CmpOp::Gt | CmpOp::GtE],
                [
                    Expr::NumberLiteral(ast::ExprNumberLiteral {
                        value: ast::Number::Int(_),
                        ..
                    }),
                ],
            ) = (ops, comparators)
            {
                checker.report_diagnostic_if_enabled(SysVersionInfoMinorCmpInt, left.range());
            }
        }

        _ => {}
    }

    if is_sys(left, "version", checker.semantic()) {
        if let (
            [CmpOp::Lt | CmpOp::LtE | CmpOp::Gt | CmpOp::GtE],
            [Expr::StringLiteral(ast::ExprStringLiteral { value, .. })],
        ) = (ops, comparators)
        {
            if value.len() == 1 {
                checker.report_diagnostic_if_enabled(SysVersionCmpStr10, left.range());
            } else {
                checker.report_diagnostic_if_enabled(SysVersionCmpStr3, left.range());
            }
        }
    }
}
