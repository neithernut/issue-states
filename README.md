issue-states -- manage the states of issues

This repository houses the specification of "issue states" as well as a
reference implementation of a library providing an associates data model and
resolution logic.

An issue state is a named transient property depending on an issue's metadata.
E.g., rather than a property which is actively controlled by an issue tracker's
admin or user, it is a property which is determined from the issue's metadata.

An issue's state is selected out of a set of issue states, which may either be
hard-coded in an application (discouraged) or specified by some configuration.
For a given issue, each of those states is either enabled or disabled based on a
"condition" attached to the state. Additionally, a state may or may not be
related to other issues within the set.

For a given issue, a resolver may select at most one of the states enabled for
the issue.

# Motivation

In issue- and bugtrackers, issues are usually assigned a status of some sort,
e.g. "open" or "closed". Some issue tracker feature a more rich set of states
as well as (sometimes customizable) workflows: a set of possible states and
transitions specified by the admin or project maintainer.

Workflows are, essentially, specialized state machines and libraries exist for
implementing workflow behavior. However, those are often targeted at a specific
environment (e.g. a specific CMS) and generally do not provide any specification
format for state machines. Rather, workflows are constructed (more or less)
programatically through source code. Persisting, if supported at all, usually
only targets databases --using custom database schemas.

We primarily require a specification format for issue-related workflows. As no
widely used format appears to exist, we design our own. We will also develop a
library for parsing the format and implementing the workflow's behavior.

