# Conditions

A condition is composed of atoms. Each atom represents a "singular" condition on
a specific piece of metadata. How atoms are composed to more complex conditions
is in the scope of the representation format. For example, a condition may be an
arbitrary Boolean expression in one format. In another format, a condition may
be a sequence of atoms, representing the conjunction of the individual atoms.


## Condition atoms

The central element of a condition atom is a "metadata identifier" denoting the
piece of metadata on which the condition is applied. The identifier must not
contain any of `!`, `<`, `>`, `=`, or `~` and must also contain no white-space
characters. Additional constrains on the characters may be imposed by the format
in which the issue states are represented.

A condition may consist of either
 * only a metadata identifier,
 * a "negator" immediately followed by a metadata identifier or
 * a metadata identifier immediately followed by a "match operator" immediately
   followed by an arbitrary "value".

A match operator is one of `=`, `<`, `>`, `<=`, `>=` or `~`, optionally preceded
by a negator. A negator is the `!` character. A "value" is a representation of
an object (or literal) which allows comparison to the piece of metadata referred
to by the metadata identifier.


## Match operators

A match operator is a binary operator representing a relation between two
values. In a condition atom, the left hand side is the value of the metadata
denoted by the metadata identifier. The right hand side is the value contained
in the condition as a literal. The following operators are defined:

 * `=` (equality): true if both sides of the operator are equal.
 * `<` (lower than): true if the left-hand side is "lower" than the right-hand
   side. If the left-hand side is "lower than" the right-hand side, it must not
   be equal to the right-hand side.
 * `>` (greater than): true if the left-hand side is "lower" than the
   right-hand side. The relation represented by this operator must be equivalent
   to the `>` operator if the left- and the right-hand side were swapped.
 * `<=` (lower than or equal): true if the left-hand side is lower than or equal
   to the right-hand side. The relation represented by this operator must be the
   disjunction of both `<` and `=`.
 * `>=` (greater than or equal): true if the left-hand side is greater than or
   equal to the right-hand side. The relation represented by this operator must
   be the disjunction of both `>` and `=`.
 * `~` (contains): true if the left-hand side "contains" the right-hand side,
   also if the left-hand side and the right-hand side are equal. `~` and `=` may
   be equivalent for a given metadata type.

Note that the operators `=`, `<`, `>`, `<=` and `>=` represent common ordering
relations of a partially ordered set. E.g. if these operators are provided for
a given "type" of metadata values, the values of that type represent a partially
ordered set.

If a relation is not defined for a given type, an implementation may either
generate an error or substitute an empty relation, e.g. the operator never
matches. However:

 * If equality (`=`) is defined, "contains" (`~`) must also be defined.
 * If "lower than" (`<`) is defined, so must be "greater than" (`>`) and
   vise-versa.
 * Provided that equality `=` is defined, then if `<` and `>` is defined, `<=`
   and `>=` must also be defined and vise-versa.

As described above, each of the operators above can be negated by prepending a
negator (`!`).


## A note on the value

Obviously, the value is directly preceded by the match operator. This imposes
constraints on the values (or rather, the string representations) possible.
However, since the match operator consists of at least one character, only the
match operators consisting of multiple characters constrain possible values. All
of those operators end with a `=`. Hence, a value cannot start with a `=`, but
with any other character.

