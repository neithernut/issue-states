# Semantics

The central objects of interest are the states an issue may have, henceforth
referred to as "issue states". However, in this project, issue states are not
a property which is actively assigned to an issue but rather a property which is
inferred from its metadata, based on the specification. For example, an issue
may exhibit the state "assigned" if an assignee is present for the issue. An
issue state is either "engaged" or "disengaged".

An issue state is associated with a "condition", which is conceptually a pure
function mapping an issue's metadata to a truth value. A state is said to be
"enabled" for a given issue if this function yields "true" for the issue's
metadata, e.g. if the condition is satisfied.

An issue exhibits at most one state at a given time. This state has to be one of
the enabled states for the issue. Contrary to common workflow concepts, state
transitions are not modeled --hence the name "issue-states" rather than
something containing "workflow". Thus, no restrictions or actions can be
associated with an transition. If a user can arbitrarily alter issue metadata,
it can be assumed that he or she may put the issue in any state specified.


## Relations between states

An issue state may "override" one or more other issue states if specified:
if the state is enabled, the issue cannot exhibit any of the states overridden.
For example, if two states are enabled for an issue but one overrides the other,
the overriding state is selected as the issue's state. The relation "overrides"
is both anti-symmetric and transitive.

An issue state may "extend" on one or more other issue states. This relation
has the same effect as the "override" relation. However, the extending state
inherits the conditions of the extended states, e.g. the state is only enabled
if its own condition is satisfied and all states it extends are enabled. The
inheritance of extended states is transitive with regard to the relation
"extends". Thus, the relation "depends on" is also both anti-symmetric and
transitive.

The relation given by the union of the relations "overrides" and "extends" shall
also be anti-symmetric and transitive.


## Note on computability

The rules above are designed to keep the resolution of an issue's state
computable, e.g. it should not be possible to construct contradictions using the
relations between issue-states.

