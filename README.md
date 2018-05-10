issue-states -- manage the states of issues

**We are in the process of defining the scope and config format. No code yet.**

# Motivation

In issue- and bugtrackers, issues are usually assigned a status of some sort,
e.g. "open" or "closed". Some issue tracker feature a more rich set of states
as well as (sometimes customizable) workflows: a set of possible states and
transitions specified by the admin or project maintainer.

Workflows are, essentially, specialized state machines and libraries exist for
implementing workflow behavior. However, those are often targeted at a specific
environment (e.g. a specific CMS) and generally do not provide any specification
format for state machines. Rather, workflows are constructed (more or less)
programaticaly through source code. Persisting, if supported at all, usually
only targets databases --using custom database schemas.

We primarily require a specification format for issue-related workflows. As no
widely used format appears to exist, we design our own. We will also develop a
library for parsing the format and implementing the workflow's behavior.

