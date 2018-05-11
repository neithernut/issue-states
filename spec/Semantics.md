# Semantics

The central objects of interest are the states an issue may have, henceforth
referred to as "issue states". However, in this project, issue states are not
a property which is actively assigned to an issue but rather a property which is
inferred from its metadata, based on the specification. For example, an issue
may exhibit the state "assigned" if an assignee is present for the issue. An
issue state is either "engaged" or "disengaged".

Contrary to what is common for issue- and bugtrackers, multiple states may be
engaged for an issue, e.g. if the metadata satisfies the specifications of
multiple issue states. Also contrary to common workflow concepts, state
transitions are not modeled --hence the name "issue-states" rather than
something containing "workflow". Thus, no restrictions or actions can be
associated with an transition. If a user can arbitrarily alter issue metadata,
it can be assumed that he or she may put the issue in any state specified.


## Relations between states

An issue state may "override" one or more other issue states if specified:
if a state is "engaged" due to the issue's metadata, states overridden by
this state are "disengaged", e.g. no longer visible to the user regardless of
whether the metadata satisfies the state's perquisites. The relation "overrides"
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

