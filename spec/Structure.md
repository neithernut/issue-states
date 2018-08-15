# Structure

An issue state specification consist of a list or sequence of descriptions of
issue state. Each issue state is identified by a unique, mandatory name.
It may contain a specification for the metadata conditions as well as references
to other issue states extended and overridden by the state.


## YAML 1.2

A self-contained set of issue states may be specified using a YAML 1.2 document.
As a file may contain multiple YAML documents or other context, an issue state
specification may coexist with other context within a file. This property of
YAML may, for example, be used to store both issue state and metadata access
control specifications in a single file.

The top-level node of an issue state specifying YAML document is a sequence.
Each item of that sequence describes one issue state.

In the most simple case, the item consists of the state's name, represented as a
string. Such an item represents a state which is unconditionally enabled. In any
other case, the item consists of a map describing the state.

 * The mandatory entry with the key `name` denotes the issue state's name. The
   value of this entry is the name
 * The optional entry with the key `condition` denotes the metadata condition.
   The value of this entry is a list of strings containing conditions for single
   pieces of metadata in an implementation defined format. If the condition of
   the state is expressible using a single string, this string may be used in
   place of a list containing only one item. The state's condition is the
   conjunction of all the sub-conditions expressed through the individual
   strings.
 * The optional entry with the key `extends` denotes states which are extended
   by the current state. The value of this entry is a list of strings, each
   matching an issue-state's name. Alternatively, if the state depends only on a
   single other state, a single string may be used.
 * The optional entry with the key `overrides` denotes states which are
   overridden by the current state. Like for the `extends` entry, its value is a
   list of state names or a single state name.

