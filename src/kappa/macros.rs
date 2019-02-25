use super::Agent;

#[allow(unused)]
macro_rules! agent {

    // A()
    ($name:ident()) => ({
        $crate::kappa::Agent::new(stringify!($name))
    });
    // A(x)
    ($name:ident ( $site:ident, $($rest:tt)* ) ) => ({
        let mut agent = agent!($name());
        __agent_impl_sites!(agent,  $($sites)*);
        agent
    });

    ($name:ident($site:ident { $($states:ident),* } )) => ({
        let mut agent = agent!($name());
        agent.site(site!($site {$($states),*}));
        agent
    });

    ($name:ident($site:ident [$($links:tt)*] )) => ({
        let mut agent = agent!($name());
        agent.site(site!($site [$($links)*]));
        agent
    });

    ($name:ident($site:ident {$($states:ident),*}, $($rest:tt)* )) => ({
        agent!($name($site {$($states),*} [], $($rest)* ))
    });

    ($name:ident($site:ident [$($links:tt)*], $($rest:tt)* )) => ({
        agent!($name($site {} [$($links)*], $($rest)* ))
    });

    ($name:ident($site:ident {$($states:ident),*} [$($links:tt)*], $($rest:tt)* )) => ({
        let mut agent = agent!($name());
        agent.site(site!($site {$($states),*} [$($links)*]));
        __agent_impl_sites!(agent, $($rest)*);
        agent
    });


    // // A(s{s1})
    // ($name:ident($site:ident {$($states:tt)*})) => ({
    //     let mut agent = agent!($name());
    //     // let mut site = site!($site {$($states)* []});
    //     // agent.site(site);
    //     agent
    // });
    // // A(s{s1}[a.B])
    // ($name:ident($site:ident {$($states:tt)*} [$($links:tt)*])) => ({
    //     let mut agent = agent!($name());
    //     // let mut site = site!($site {$($states)*} [$($links)*]);
    //     // agent.site(site);
    //     agent
    // });

    // A(s{s1}[a.B], s{s2}[a.C])
    // ($name:ident( $($rest:tt)* )) => ({
    //     let mut agent = agent!($name());
    //     agent!(@agent $name ($($rest)*));
    //     agent
    // });


    // (@$agent:ident $name:ident($site:ident {$($states:tt)*} [$($links:tt)*], $($rest:tt)*) ) => ({
    //     // agent.site(site!($site {$($states)*} [$($links)*]));
    //     agent!(@site $name ($($rest)*) );
    // });

    // (@$agent:ident $name:ident()) => ();



}

#[doc(hidden)]
macro_rules! __agent_impl_sites {
    ($agent:ident, $name:ident) => ({
        $agent.site(site!($name));
    });
    ($agent:ident, $name:ident {$state:ident}) => ({
        $agent.site(site!($name {$state}));
    });
    ($agent:ident, $name:ident [$($link:tt)*]) => ({
        $agent.site(site!($name [$($link)*]));
    });
    ($agent:ident, $name:ident {$($states:ident),*}) => ({
        $agent.site(site!($name { $($states),* }));
    });

    ($agent:ident, $name:ident {$state:ident}, $($rest:tt)*) => ({
        $agent.site(site!($name {$state}));
        __agent_impl_sites!($agent, $($rest)*)
    });
    ($agent:ident, $name:ident [$($link:tt)*], $($rest:tt)*) => ({
        $agent.site(site!($name [$($link)*]));
        __agent_impl_sites!($agent, $($rest)*)
    });
    ($agent:ident, $name:ident {$($states:ident),*}, $($rest:tt)*) => ({
        $agent.site(site!($name { $($states),* }));
        __agent_impl_sites!($agent, $($rest)*)
    });


    // ($agent:ident, $name:ident {$($state:ident),*} [$($link:tt)*]) =>
}





#[allow(unused)]
macro_rules! site {
    ($name:ident) => ($crate::kappa::Site::new(stringify!($name)));
    ($name:ident []) => (site!($name));
    ($name:ident {}) => (site!($name));
    ($name:ident {} []) => (site!($name));
    ($name:ident {$state:ident}) => (site!($name {$state} []));
    ($name:ident {$($state:ident),*}) => ({site!($name {$($state),*} [])});
    ($name:ident [$($link:tt)*]) => ({
        let mut site = site!($name);
        __site_impl_links!(site [$($link)*]);
        site
    });
    ($name:ident { $($state:ident),* } []) => ({
        let mut site = site!($name);
        $(site.state(stringify!($state));)*
        site
    });
    ($name:ident {$($state:ident),*} [$($link:tt)*]) => ({
        let mut site = site!($name);
        $(site.state(stringify!($state));)*
        __site_impl_links!(site [$($link)*]);
        site
    });
}


#[doc(hidden)]
macro_rules! __site_impl_links {
    ($site:ident []) => ();
    ($site:ident [#]) => ({$site.link(link!(#));});
    ($site:ident [.]) => ({$site.link(link!(.));});
    ($site:ident [_]) => ({$site.link(link!(_));});
    ($site:ident [$other:ident . $agent:ident]) => ({
        $site.link(link!($other . $agent));
    });
    ($site:ident [$n:expr]) => ({$site.link(link!($n));});
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
    ($site:ident [$n:expr, $($links:tt)*]) => ({
        $site.link(link!($n));
        __site_impl_links!($site [$($links)*])
    });
}


#[allow(unused)]
macro_rules! link {
    (.) => {$crate::kappa::Link::Free};
    (#) => {$crate::kappa::Link::Unknown};
    (_) => {$crate::kappa::Link::Bound};
    ($site:ident . $agent:ident) => {
        $crate::kappa::Link::BoundTo {
            agent: stringify!($agent).to_string(),
            site: stringify!($site).to_string(),
        }
    };
    ($site:expr) => {$crate::kappa::Link::Numbered($site)};
}
