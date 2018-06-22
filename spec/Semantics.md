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
is both anti-symmetric and transitive. Since an issue state should naturally
never override itself, the graph given by issue states and the relation should
be free of cycles.

An issue state may also "depend" on one or more other issue states, e.g. the
state is only engaged if all issue states it depends on are engaged. Like the
relation "overrides", the relation "depends on" is both anti-symmetric and
transitive. Also, the corresponding graph should be free of cycles. An issue
cannot depends on an issue and override the same issue at the same time.


## Names

States have a name, which is part of the state's specification. A state's
specification may also contain the name of a "counter-state". This introduces a
pseudo state which is engaged if, and only if, the original state is disengaged.
A state cannot depend or override a counter-state directly or transitively.


## Note on computability

The rules above are designed to keep the resolution of an issue's state
computable, e.g. it should not be possible to construct contradictions using the
relations between issue-states.

