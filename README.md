# Cerebellum

A semi-functional, array oriented golfing language with no cheaty builtins. Currently has 254 operators.

This language will use polish notation, so will be unique in being not stack based. There are only 5 types:

- Numbers, can be float or integer. Program will decide for you.
- Lists/Iterators, can be infinite or finite. Are lazy, will only be evaluated when needed. Applying arithmetic to arrays vecotrizes.
- Functions - Take either 1, 2, or 3 arguments. Can only be called with the special "call function" operation. Applying arithmetic to functions composes, aka `(λa:x)+(λa:y) = (λa:x+y)`
- True, equivalent to 1 is most situations
- False/None, equivalent to 0 in most situations

Other types are represented as follows:

- Characters are just numbers
- Strings are just lists of characters
- Note: Many string functions will treat lists of characters as or. Thus `replaceChar("abc")` will actually replace on any `a`,
`b` OR `c`, while `replaceString("abc")` will only replace if the characters appear in a row.
- Numbers can be considered arrays of their decimal digits

This also means many, if not most string functions will work on arrays of other data too, including arrays of arrays.

All types are immutable, all operations make a copy. For example, using `setN` on a list will actually return a new list with the `n-th`
element replaced, leaving the old one unchanged.

All operations have fixed arity. Only the `Splat` and `SplatSingle` commands can override this.

## Infinite Arrays (0)

I'm still thinking a bit about the implementation details of the infinite arrays, and how to handle them efficiently, while also staying inituitive. In general there will be at least
a couple different categoires:

- Fully Filled Arrays, size known, all elements precomputed
- Generators - No ellements filled, items evaluated successively. May be infinite, finite, or unknown size.
- Mapping Functions - Like generators but don't require previous values known to compute the next. Still lazy. May be infinite, finite, or of unknown size. Because of the limited amount of possible side effects it will be reasonably to dynamically decide if a certain list is a generator or mapping function.
- Addons: A mutation on a existing array, eg: Using Insert, Set or Delete

Iterator methods will be the only way to really do looping so forcing evaluation of a map must be super convenient. I propose immediatly evaluating any iterator command that
contains a side effect. This way the behavior will always appear identical to a non-lazy list. However, still supporting infinite values. Mapping infinite arrays with side effects
simply leads to a infinite loop.

This does mean that creating a infinte loop with no side effects becomes harder, though not impossible. Either use the IsFinite built on or just add a side effect that effectively does nothing.

The next issue is handling multiple copies of lazy things. Since everything is "immutable" we need to copy the array every time. We don't want a huge performance penalty from
lazy evaluating things multiple times. Copies should still probably really be a reference to the same underlying object, storing it's own changes seperately. Then when one gets evaluated they all get evaluated. At some point a list could have so many changes though it's more effort to keep track of the changes than to just flatten the original list. It could be tricky to find out when to do this, especially when you have no idea how expensive evaluting the expression actually is.

This is in addition to the standard `CopyOnWrite` optimization for non-lazy lists.

## Iterator Commands (9)

These functions will implititly create a new function. No lambda needed. Note: Lists may be infinite.

* Map (Might be not neded, implicit?)
* Filter
* Reduce
* Scan
* Successors
* FixedPoint (Apply till unchanged)
* Takewhile
* Any
* All

## Iterator -> Scalar (11)

Everything is immutable, so eveything will create a new list.

* Length
* First
* Last
* Nth
* Slice/Sublist/Substring - Actually returns a iterator.
* Sum
* All Equal?
* Count (Same as Filter Length)
* Average
* Median
* Mode

## Iterator -> Iterator (31)

* Truncate
* Chain (concat)
* ChainInfinite
* Flatten
* Delete Nth
* Replace Nth
* Insert at N
* Insert at start
* Insert at end
* Delete Range
* Replace Range
* Chunks
* Transpose
* ChunksOverlapping
* Zip
* ZipSelf
* Enumerate
* Union
* Intersection
* Sort
* SortReverse
* Sort by key
* Intersperse
* FilterUnique
* Uniquefy
* Local Maxima
* To Sparse
* From Sparse
* Reverse
* RotateLeft
* RotateRight

## Primite -> Iterator (7)

* Range
* InfiniteRange
* RangeInclusive
* RangeWithStep
* N-Sized (Takes a array of size N, returns a N-dimentional array with coordinates equal to the values in the array. Fill with X)
* Once
* Pair

## Python Itertools (6)

* Combinations
* Permutations
* Product
* Nth Self Product
* Powerset
* Sublists



## Arithmetic (33)

Vectorizes arrays, composes functions.

* `+`
* `-`
* `*`
* float division
* integer division
* `%`
* `&`
* `|`
* `^`
* Negate
* Reprical
* Power
* Log2
* Log10
* LogE
* LogN
* Sin
* Cos
* Tan
* asin
* atan
* Increment
* Decrement
* abs()
* Sqrt
* nth-root
* Factorial
* floor
* ceil
* round
* To degrees
* To radians
* 1 Count (binary)

## Logic (13)

* If
* Or (Short Cirquiting)
* And (Short Cirquiting)

## Variables (16)

* Most Recently Defined
* Second
* Third
* Fourth
* Fifth
* Sixth
* N-th variable
* List of command line arguments. Excludes any used by the interpreter.
* Create Variable
* Create function variable (lambda)
* Index in inner loop
* Index in parent loop
* Index in nth-parent loop
* Parent List
* Parent Parent List
* Nth-parent list

## Functions (6)

* Create a new anynymous function
* Call function with 1 argument, others will be set to None
* Call function with 2 arguments, others will be set to None
* Call function with 3 arguments, others will be set to None
* Recurse
* Recurse File

## Constants (25)

* 1
* 2
* 3
* 4
* 5
* 6
* 7
* 8
* 9
* 10
* 11
* 12
* 13
* 14
* 15
* 16
* PI
* E
* Infinity
* 10
* 100
* 1000
* 256
* 512
* None

## Strings (28)

* SplitChar
* SplitString
* SplitWhitespace
* SplitWith
* SplitOnceChar
* SplitOnceString
* SplitOnceWith
* Upper
* Lower
* ContainsChar
* ContainsString
* ReplaceChar
* ReplaceString
* ReplaceOnceChar
* ReplaceOnceString
* ReplaceStringWith
* ReplaceCharWith
* StartsWithChar
* EndsWithChar
* Strip
* PadLeft
* PadLeftWith
* PadRight
* PadRightWith
* PadCenter
* PadCenterWith
* Join
* Matching Parenthesis

## Strings Constants (8)

* Letters
* LowercaseLetters
* UppercaseLetters
* Symbols
* Printable
* Whitespace
* Empty String
* Digits


## IO (10)

* Output as letters (Unicode code points)
* Output as numbers (repr)
* Print a space
* Print a line break
* Input as string (list of char codes)
* Input as list of numbers (decimals, space seperated)
* Exit
* Current Time (Unix Time)
* Current Time (ISO)
* Current Time `[year, month, day, hour, minute, second, millisecond]`

## Logic (13)

* Equals
* Less
* Greater
* LessOrEqual
* GreaterOrEqual
* Not
* Min
* Max
* Min2
* Max2
* IsNone
* InRange (Incl)
* InRange (Excl)

## Literals (4)

* Number Literal (base64)
* String Literal (Raw)
* String Literal (base255)
* String Literal (ASCII printable Only)

## Parenthesis (2)

* Break
* Continue

## Inspection (7)

* Source Code
* Eval
* Filename
* Read file
* Write to file
* Read HTTP
* Post HTTP

## Linear Algebra (9)

* Dot Product
* Cross Product
* Complex Multiply (Padded to power of 2, max 8)
* Complex Divide
* Matrix Multiply
* Diagonal
* Antidiagonal
* Matrix Inverse
* Complex to matrix

## Numbers to strings (7)

* Stringify (repr's lists, decimals numbers)
* number to base 16
* number to base N
* string to number (base 10)
* String to number, n decimal places (base 10)
* stirng to number (base 16)
* string to number (base N)

## Primes (5)

* Infinite List of Primes
* Factors
* Prime Factors
* GCD
* LCM

## Random (5)

* Choice
* Number in range
* Float
* Condition
* Shuffle

## Special (5)

* Splat (apply a array to multiple arguments of a function, overrides arity)
* SplatSingle (apply a single value to all arguments of a function)
* SplatVariables (In a map apply the function with the variables from the current lambda)
* IsFinite: Return the list evaluated if it's finite. Otherwise, return None. Also works on numbers. If the list is infinite but can't be proven (eg. you are trying to solve 3n+1) this might hang forever. Will also hang forever for generators with side effects. For numbers returns if the number is finite. For functions returns if it's O(1).
* Discard: Evaluate a element but return None



