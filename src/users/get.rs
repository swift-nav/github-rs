//! Access the Users portion of the GitHub API
imports!();
use client::GetQueryBuilder;

// Declaration of types representing the various items under users
new_type!(
    Emails
    Followers
    Following
    FollowingUser
    Keys
    KeysId
    Orgs
    User
    Users
    UsersOrgs
    Events
    EventsOrgs
    EventsOrgsName
    Gists
    UsersKeys
    UsersStarred
    UserUsername
    UsersUsername
    Repos
    ReceivedEvents
    Issues
    IssuesFilter
    IssuesState
    IssuesLabels
    IssuesSort
    IssuesDirection
    IssuesSince
    Public
    Starred
    StarredRepo
    StarredOwner
    Subscriptions
);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> User  = "user"
        -> Users = "users"
    @Events
        -> EventsOrgs = "orgs"
        -> Public =  "public"
    @EventsOrgs
        => EventsOrgsName
    @Following
        => FollowingUser
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
    @Keys
        => KeysId
    @Starred
        => StarredOwner
    @StarredOwner
        => StarredRepo
    @User
        -> Emails = "emails"
        -> Followers = "followers"
        -> Following = "following"
        -> Keys = "keys"
        -> Issues = "issues"
        -> Orgs = "orgs"
        -> Repos = "repos"
        -> Subscriptions = "subscriptions"
        -> Starred = "starred"
    @Users
        => UsersUsername
    @UserUsername
        -> Followers = "followers"
        -> Following = "following"
        -> UsersKeys = "keys"
        -> Repos = "repos"
    @UsersUsername
        -> Followers = "followers"
        -> Following = "following"
        -> Events = "events"
        -> Gists = "gists"
        -> UsersOrgs = "orgs"
        -> UsersKeys = "keys"
        -> Repos = "repos"
        -> Subscriptions = "subscriptions"
        -> UsersStarred = "starred"
        -> ReceivedEvents = "received_events"
);

// impls of each type
impl_macro!(
    @Starred
        |
        |=> owner -> StarredOwner = owner_str
    @StarredOwner
        |
        |=> repo -> StarredRepo = repo_str
    @User
        |=> emails -> Emails
        |=> followers -> Followers
        |=> following -> Following
        |=> issues -> Issues
        |=> repos -> Repos
        |=> subscriptions -> Subscriptions
        |=> starred -> Starred
        |=> keys -> Keys
        |=> orgs -> Orgs
        |
    @Users
        |
        |=> username -> UsersUsername = username_str
    @UserUsername
        |=> followers -> Followers
        |=> following -> Following
        |=> keys -> UsersKeys
        |=> repos -> Repos
        |
    @UsersUsername
        |=> events -> Events
        |=> followers -> Followers
        |=> following -> Following
        |=> gists -> Gists
        |=> orgs -> UsersOrgs
        |=> keys -> UsersKeys
        |=> received_events -> ReceivedEvents
        |=> repos -> Repos
        |=> starred -> UsersStarred
        |=> subscriptions -> Subscriptions
        |
    @Events
        |=> orgs -> EventsOrgs
        |=> public -> Public
        |
    @EventsOrgs
        |
        |=> org -> EventsOrgsName = org_name_str
    @Keys
        |
        |=> id -> KeysId = id_str
    @Following
        |
        |=> username -> Following = username_str
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

exec!(Emails);
exec!(Events);
exec!(EventsOrgsName);
exec!(Followers);
exec!(Following);
exec!(FollowingUser);
exec!(Gists);
exec!(Issues);
exec!(IssuesFilter);
exec!(IssuesState);
exec!(IssuesLabels);
exec!(IssuesSort);
exec!(IssuesDirection);
exec!(IssuesSince);
exec!(Keys);
exec!(KeysId);
exec!(Orgs);
exec!(Public);
exec!(ReceivedEvents);
exec!(Repos);
exec!(Starred);
exec!(StarredRepo);
exec!(Subscriptions);
exec!(User);
exec!(UserUsername);
exec!(Users);
exec!(UsersKeys);
exec!(UsersOrgs);
exec!(UsersStarred);
exec!(UsersUsername);
