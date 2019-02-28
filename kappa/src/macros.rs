//! Quasi-quoting macros emulating the Kappa syntax.

#[macro_export]
macro_rules! rule {
    // 'rule' {} => {}
    ($name:literal $left:tt => $right:tt @ $rate:expr) => ({
        let mut rule = $crate::Rule::with_name($name, $rate);
        __rule_impl_slots!(rule, $left => $right);
        rule
    });
    // ?rule {} => {}
    (? $name:ident $left:tt => $right:tt @ $rate:expr) => ({
        let mut rule = $crate::Rule::with_name($name, $rate);
        __rule_impl_slots!(rule, $left => $right);
        rule
    });
    // {} => {}
    ($left:tt => $right:tt @ $rate:expr) => ({
        let mut rule = $crate::Rule::new($rate);
        __rule_impl_slots!(rule, $left => $right);
        rule
    });
}

#[macro_export]
#[doc(hidden)]
macro_rules! __rule_impl_slots {
    // {} => {}
    ($rule:ident, {} => {}) => ();
    // {A(...), ...} => {A(...), ...}
    ($rule:ident, { $l:ident ($($largs:tt)*), $($lrest:tt)* } => {  $r:ident ($($rargs:tt)*), $($rrest:tt)* }) => ({
        __rule_impl_slots!($rule, { $l ($($largs)*) } =>  { $r ($($rargs)*) });
        __rule_impl_slots!($rule, { $($lrest)* } => { $($rrest)* });
    });
    // {A(...)} => {A(...)}
    ($rule:ident, { $l:ident ($($largs:tt)*) } => { $r:ident ($($rargs:tt)*) } ) => ({
        $rule.slot(agent!($l($($largs)*)), agent!($r($($rargs)*)))
    });
    // {A(...),} => {A(...)}
    ($rule:ident, { $l:ident ($($largs:tt)*), } => { $r:ident ($($rargs:tt)*) } ) => ({
        __rule_impl_slots!( $rule, { $l ($($largs)*) } => { $r ($($rargs)*) });
    });
    // {A(...)} => {A(...),}
    ($rule:ident, { $l:ident ($($largs:tt)*) } => { $r:ident ($($rargs:tt)*), } ) => ({
        __rule_impl_slots!( $rule, { $l ($($largs)*) } => { $r ($($rargs)*) });
    });
    // {A(...),} => {A(...),}
    ($rule:ident, { $l:ident ($($largs:tt)*), } => { $r:ident ($($rargs:tt)*), } ) => ({
        __rule_impl_slots!( $rule, { $l ($($largs)*) } => { $r ($($rargs)*) });
    });
}

#[macro_export]
macro_rules! agent {
    // (?) A()
    ($name:ident()) => ({
        $crate::Agent::new(stringify!($name))
    });
    (? $name:ident()) => ({
        $crate::Agent::new($name)
    });
    // (?) A(x)
    ($name:ident ( $site:ident, $($rest:tt)* ) ) => ({
        let mut agent = agent!($name());
        __agent_impl_sites!(agent,  $($sites)*);
        agent
    });
    (? $name:ident ( $site:ident, $($rest:tt)* ) ) => ({
        let mut agent = agent!(? $name());
        __agent_impl_sites!(agent,  $($sites)*);
        agent
    });
    // (?) A(x{...})
    ($name:ident( $site:ident { $($states:tt)* } )) => ({
        let mut agent = agent!($name());
        agent.site(site!($site {$($states)*}));
        agent
    });
    (? $name:ident( $site:ident { $($states:tt)* } )) => ({
        let mut agent = agent!(? $name());
        agent.site(site!($site {$($states)*}));
        agent
    });
    // (?) A(x[...])
    ($name:ident($site:ident [$($links:tt)*] )) => ({
        let mut agent = agent!($name());
        agent.site(site!($site [$($links)*]));
        agent
    });
    (? $name:ident($site:ident [$($links:tt)*] )) => ({
        let mut agent = agent!(? $name());
        agent.site(site!($site [$($links)*]));
        agent
    });
    // (?) A(x{...}, ...)
    ($name:ident($site:ident {$($states:ident),*}, $($rest:tt)* )) => ({
        agent!($name($site {$($states),*} [], $($rest)* ))
    });
    (? $name:ident($site:ident {$($states:ident),*}, $($rest:tt)* )) => ({
        agent!(? $name($site {$($states),*} [], $($rest)* ))
    });
    // (?) A(x[...], ...)
    ($name:ident($site:ident [$($links:tt)*], $($rest:tt)* )) => ({
        agent!($name($site {} [$($links)*], $($rest)* ))
    });
    (? $name:ident($site:ident [$($links:tt)*], $($rest:tt)* )) => ({
        agent!(? $name($site {} [$($links)*], $($rest)* ))
    });
    // A(x{...}[...], ...)
    ($name:ident($site:ident {$($states:ident),*} [$($links:tt)*], $($rest:tt)* )) => ({
        let mut agent = agent!($name());
        agent.site(site!($site {$($states),*} [$($links)*]));
        __agent_impl_sites!(agent, $($rest)*);
        agent
    });
    (? $name:ident($site:ident {$($states:ident),*} [$($links:tt)*], $($rest:tt)* )) => ({
        let mut agent = agent!(? $name());
        agent.site(site!($site {$($states),*} [$($links)*]));
        __agent_impl_sites!(agent, $($rest)*);
        agent
    });
}

#[macro_export]
#[doc(hidden)]
macro_rules! __agent_impl_sites {
    // (?) r
    ($agent:ident, $name:ident) => ({
        $agent.site(site!($name));
    });
    ($agent:ident, ? $name:ident) => ({
        $agent.site(site!(? $name));
    });
    // (?) r[...]
    ($agent:ident, $name:ident [$($link:tt)*]) => ({
        $agent.site(site!($name [$($link)*]));
    });
    ($agent:ident, ? $name:ident [$($link:tt)*]) => ({
        $agent.site(site!(? $name [$($link)*]));
    });
    // (?) r{...}
    ($agent:ident, $name:ident {$($states:tt)*}) => ({
        $agent.site(site!($name {$($states)*}));
    });
    ($agent:ident, ? $name:ident {$($states:tt)*}) => ({
        $agent.site(site!(? $name {$($states)*}));
    });
    // (?) r[...], ...
    ($agent:ident, $name:ident [$($link:tt)*], $($rest:tt)*) => ({
        $agent.site(site!($name [$($link)*]));
        __agent_impl_sites!($agent, $($rest)*);
    });
    ($agent:ident, ? $name:ident [$($link:tt)*], $($rest:tt)*) => ({
        $agent.site(site!(? $name [$($link)*]));
        __agent_impl_sites!($agent, $($rest)*);
    });
    // (?) r{...}, ...
    ($agent:ident, $name:ident {$($states:tt)*}, $($rest:tt)*) => ({
        $agent.site(site!($name {$($states)*}));
        __agent_impl_sites!($agent, $($rest)*);
    });
    ($agent:ident, ? $name:ident {$($states:tt)*}, $($rest:tt)*) => ({
        $agent.site(site!(? $name {$($states)*}));
        __agent_impl_sites!($agent, $($rest)*);
    });
    // (?) r{...}[...], ...
    ($agent:ident, $name:ident {$($states:tt)*} [$($link:tt)*], $($rest:tt)*) => ({
        $agent.site(site!($name [$($link)*]));
        __agent_impl_sites!($agent, $($rest)*);
    });
    ($agent:ident, ? $name:ident {$($states:tt)*} [$($link:tt)*], $($rest:tt)*) => ({
        $agent.site(site!(? $name [$($link)*]));
        __agent_impl_sites!($agent, $($rest)*);
    });
}

#[macro_export]
macro_rules! site {
    // (?) r
    ($name:ident) => ($crate::Site::new(stringify!($name)));
    (? $name:ident) => ($crate::Site::new($name));
    // (?) r{...}
    ($name:ident {$($states:tt)*}) => ( site!($name { $($states)* } []) );
    (? $name:ident {$($states:tt)*}) => ( site!(? $name { $($states)* } []) );
    // (?) r[...]
    ($name:ident [$($link:tt)*]) => ( site!($name {} [$($link)*]) );
    (? $name:ident [$($link:tt)*]) => ( site!(? $name {} [$($link)*]) );
    // (?) r{...}[...]
    ($name:ident {$($states:tt)*} [$($link:tt)*]) => ({
        let mut site = site!($name);
        __site_impl_links!(site [$($link)*]);
        __site_impl_states!(site {$($states)*});
        site
    });
    (? $name:ident {$($states:tt)*} [$($link:tt)*]) => ({
        let mut site = site!(? $name);
        __site_impl_links!(site [$($link)*]);
        __site_impl_states!(site {$($states)*});
        site
    });
}

#[macro_export]
#[doc(hidden)]
macro_rules! __site_impl_states {
    ($site:ident {}) => ();
    ($site:ident {$state:ident}) => ({
        $site.state(stringify!($state));
    });
    ($site:ident {? $state:ident}) => ({
        $site.state($state);
    });
    ($site:ident {$state:ident, $($rest:tt)*}) => ({
        __site_impl_states!($site {$state});
        __site_impl_states!($site {$($rest)*});
    });
    ($site:ident {? $state:ident, $($rest:tt)*}) => ({
        __site_impl_states!($site {? $state});
        __site_impl_states!($site {$($rest)*});
    });
}

#[macro_export]
#[doc(hidden)]
macro_rules! __site_impl_links {
    ($site:ident []) => ();
    ($site:ident [#]) => ({$site.link(link!(#));});
    ($site:ident [.]) => ({$site.link(link!(.));});
    ($site:ident [_]) => ({$site.link(link!(_));});
    ($site:ident [$other:ident . $agent:ident]) => (
        $site.link(link!($other . $agent))
    );
    ($site:ident [$n:expr]) => ({$site.link(link!($n));});
    ($site:ident [? $n:ident]) => ({$site.link(link!(? $n));});
    ($site:ident [#, $($link:tt)*]) => ({
        $site.link(link!(#));
        __site_impl_links!($site [$($link:tt)*])
    });
    ($site:ident [., $($link:tt)*]) => ({
        $site.link(link!(.));
        __site_impl_links!($site [$($link:tt)*])
    });
    ($site:ident [_, $($link:tt)*]) => ({
        $site.link(link!(_));
        __site_impl_links!($site [$($link:tt)*])
    });
    ($site:ident [$other:ident . $agent:ident, $($links:tt)*]) => ({
        $site.link(link!($other . $agent));
        __site_impl_links!($site [$($links)*])
    });
    ($site:ident [? $n:ident, $($links:tt)*]) => ({
        $site.link(link!(? $n));
        __site_impl_links!($site [$($links)*])
    });
    ($site:ident [$n:expr, $($links:tt)*]) => ({
        $site.link(link!($n));
        __site_impl_links!($site [$($links)*])
    });
}

#[macro_export]
macro_rules! link {
    (.) => {
        $crate::Link::Free
    };
    (#) => {
        $crate::Link::Unknown
    };
    (_) => {
        $crate::Link::Bound
    };
    ($site:ident . $agent:ident) => {
        $crate::Link::BoundTo {
            agent: stringify!($agent).to_string(),
            site: stringify!($site).to_string(),
        }
    };
    ($site:expr) => {
        $crate::Link::Numbered($site)
    };
    (? $site:ident) => {
        $crate::Link::Numbered($site)
    };
}
