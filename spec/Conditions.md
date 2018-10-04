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

