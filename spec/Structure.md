# Structure

An issue state specification consist of a list or sequence of descriptions of
issue state. Each issue state is identified by a unique, mandatory name.
It may contain one name for a counter-state as well as a specification for
the metadata conditions, dependet-on issue states and overridden issue states.


## YAML 1.2

A self-contained set of issue states may be specified using a YAML 1.2 document.
As a file may contain multiple YAML documents or other context, an issue state
specification may coexist with other context within a file. This property of
YAML may, for example, be used to store both issue state and metadata access
control specifications in a single file.

The top-level node of an issue state specifying YAML document is a sequence.
Each item of that sequence describes one issue state.

In the most simple case, the item consists of the state's name, represented as a
string. Such an item represents a state which is unconditionally engaged (unless
overridden by another state), without a counter-state.

In any other case, the item consists of a map with exactly one entry. The
entry's key represents the state's name. The value is a map representing the
state's properties. Each of those properties is optional:

 * If an entry with the key `counter` is present, the a counter-state is
   associated with the state. The value of this entry is the counter-state's
   name, represented as a single string.
 * An entry with the key `condition` denotes the metadata condition. The value
   of this entry is a list of strings containing conditions for single pieces of
   metadata in implementation defined behavior. If the condition of the state
   if expressible using a single string, this string may be used in place of a
   list containing only one item. The state's condition is the conjunction of
   all the sub-conditions expressed through the individual strings.
 * An entry with the key `depends-on` denotes other states the current state
   depends on. The value of this entry is a list of strings, each representing
   an issue-state's name. Alternatively, if the state depends only on a single
   other state, a single string may be used.
 * An entry with the key `overrides` denotes states which are overridden by the
   current state. Like for the `depends-on` entry, its value is a list of state
   names or a single state name.

