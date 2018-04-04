//! Access the Users portion of the GitHub API
#![allow(unused_imports)]
imports!();
use client::GetQueryBuilder;

// Declaration of types representing the various items under users
new_type!(
    Issues
    IssuesFilter
    IssuesState
    IssuesLabels
    IssuesSort
    IssuesDirection
    IssuesSince
);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Issues = "issues"
    @Issues
        ?> IssuesFilter = "filter"
        ?> IssuesState = "state"
        ?> IssuesLabels = "labels"
        ?> IssuesSort = "sort"
        ?> IssuesDirection = "direction"
        ?> IssuesSince = "since"
    @IssuesFilter
        ?> IssuesState = "state"
        ?> IssuesLabels = "labels"
        ?> IssuesSort = "sort"
        ?> IssuesDirection = "direction"
        ?> IssuesSince = "since"
    @IssuesState
        ?> IssuesFilter = "filter"
        ?> IssuesLabels = "labels"
        ?> IssuesSort = "sort"
        ?> IssuesDirection = "direction"
        ?> IssuesSince = "since"
    @IssuesLabels
        ?> IssuesFilter = "filter"
        ?> IssuesState = "state"
        ?> IssuesSort = "sort"
        ?> IssuesDirection = "direction"
        ?> IssuesSince = "since"
    @IssuesSort
        ?> IssuesFilter = "filter"
        ?> IssuesState = "state"
        ?> IssuesLabels = "labels"
        ?> IssuesDirection = "direction"
        ?> IssuesSince = "since"
    @IssuesDirection
        ?> IssuesFilter = "filter"
        ?> IssuesState = "state"
        ?> IssuesLabels = "labels"
        ?> IssuesSort = "sort"
        ?> IssuesSince = "since"
    @IssuesSince
        ?> IssuesFilter = "filter"
        ?> IssuesState = "state"
        ?> IssuesLabels = "labels"
        ?> IssuesSort = "sort"
        ?> IssuesDirection = "direction"
);

// impls of each type
impl_macro!(
    @Issues
        |
        |?> filter -> IssuesFilter = filter_str
        |?> state -> IssuesState = state_str
        |?> labels -> IssuesLabels = labels_str
        |?> sort -> IssuesSort = sort_str
        |?> direction -> IssuesDirection = direction_str
        |?> since -> IssuesSince = since_str
    @IssuesFilter
        |
        |?> state -> IssuesState = state_str
        |?> labels -> IssuesLabels = labels_str
        |?> sort -> IssuesSort = sort_str
        |?> direction -> IssuesDirection = direction_str
        |?> since -> IssuesSince = since_str
    @IssuesState
        |
        |?> state -> IssuesFilter = filter_str
        |?> labels -> IssuesLabels = labels_str
        |?> sort -> IssuesSort = sort_str
        |?> direction -> IssuesDirection = direction_str
        |?> since -> IssuesSince = since_str
    @IssuesLabels
        |
        |?> filter-> IssuesFilter = filter_str
        |?> state -> IssuesState = state_str
        |?> sort -> IssuesSort = sort_str
        |?> direction -> IssuesDirection = direction_str
        |?> since -> IssuesSince = since_str
    @IssuesSort
        |
        |?> filter -> IssuesFilter = filter_str
        |?> state -> IssuesState = state_str
        |?> labels -> IssuesLabels = labels_str
        |?> direction -> IssuesDirection = direction_str
        |?> since -> IssuesSince = since_str
    @IssuesDirection
        |
        |?> filter -> IssuesFilter = filter_str
        |?> state -> IssuesState = state_str
        |?> labels -> IssuesLabels = labels_str
        |?> direction -> IssuesSort = sort_str
        |?> since -> IssuesSince = since_str
    @IssuesSince
        |
        |?> filter -> IssuesFilter = filter_str
        |?> state -> IssuesState = state_str
        |?> labels -> IssuesLabels = labels_str
        |?> sort -> IssuesSort = sort_str
        |?> direction -> IssuesDirection = direction_str
);

exec!(Issues);
exec!(IssuesFilter);
exec!(IssuesState);
exec!(IssuesLabels);
exec!(IssuesSort);
exec!(IssuesDirection);
exec!(IssuesSince);
