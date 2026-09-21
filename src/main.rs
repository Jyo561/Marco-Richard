use sycamore::prelude::*;

#[component]
fn Logo() -> View {
    view! {
        a(class="brand", href="#home") {
            img(class="brand-mark", src="public/logo2.png", alt="Marco Richard")
            span(class="brand-name") {
                strong { "MARCO" }
                strong { "RICHARD" }
            }
        }
    }
}

#[component]
fn Navbar() -> View {
    let menu_open = create_signal(false);

    view! {
        header(class="navbar") {
            div(class="container nav-inner") {

                Logo()

                nav(class="nav-links") {
                    a(href="#about") { "ABOUT" }
                    a(href="#brands") { "OUR BRANDS" }
                    a(href="#world") { "OUR WORLD" }
                }

                div(class="nav-actions") {
                    a(
                        class="soft-button small",
                        href="#contact"
                    ) {
                        "CONTACT"
                        span { "↗" }
                    }

                    button(
                        class="menu-button",
                        aria-label="Open menu",
                        on:click=move |_| {
                            menu_open.set(!menu_open.get());
                        }
                    ) {
                        span {}
                        span {}
                        span {}
                    }
                }
            }
        }
    }
}


// ============================================================
// HERO
// ============================================================

#[component]
fn Hero() -> View {
    view! {
        section(class="hero section", id="home") {

            div(class="hero-orb orb-one") {}
            div(class="hero-orb orb-two") {}

            div(class="container hero-grid") {

                div(class="hero-copy") {

                    div(class="eyebrow neumorph-inset") {
                        span(class="status-dot") {}
                        "MARCO RICHARD · INDIA"
                    }

                    h1 {
                        "We make "
                        span(class="accent-text") { "moments." }
                        br {}
                        "You make them "
                        span(class="soft-text") { "matter." }
                    }

                    p(class="hero-lead") {
                        "We create beverages and brands that bring people together,
                         inspire discovery and become part of life's memorable moments."
                    }

                    div(class="hero-buttons") {

                        a(
                            class="soft-button primary",
                            href="#brands"
                        ) {
                            "EXPLORE OUR BRANDS"
                            span { "→" }
                        }

                        a(
                            class="soft-button",
                            href="#about"
                        ) {
                            "DISCOVER MARCO RICHARD"
                        }
                    }

                    div(class="hero-mini-stats") {

                        div {
                            strong { "8%+" }
                            span { "INDIA MARKET SHARE" }
                        }

                        div {
                            strong { "2024" }
                            span { "SOUTH INDIA EXPANSION" }
                        }

                        div {
                            strong { "01" }
                            span { "PURPOSE · CREATE MOMENTS" }
                        }
                    }
                }


                // Hero visual
                div(class="hero-visual") {

                    div(class="bottle-card neumorph-card") {

                        div(class="card-topline") {
                            span { "MARCO RICHARD" }
                            span { "EST. / INDIA" }
                        }

                        div(class="bottle-stage") {

                            div(class="bottle-shadow") {}

                            div(class="bottle") {
                                div(class="bottle-neck") {}
                                div(class="bottle-body") {
                                    span(class="bottle-label") {
                                        "MARCO"
                                        br {}
                                        "RICHARD"
                                    }
                                }
                            }
                        }

                        div(class="bottle-caption") {
                            span { "CRAFTED FOR" }
                            strong { "THE NEXT MOMENT" }
                        }
                    }


                    div(class="floating-chip chip-one") {
                        span(class="chip-icon") { "↗" }

                        span {
                            b { "+12%" }
                            small { "SPIRITS" }
                        }
                    }


                    div(class="floating-chip chip-two") {
                        span(class="chip-icon pink") { "✦" }

                        span {
                            b { "PREMIUM" }
                            small { "BEVERAGES" }
                        }
                    }
                }
            }

            a(class="scroll-cue", href="#about") {
                span { "DISCOVER OUR WORLD" }
                b { "↓" }
            }
        }
    }
}


// ============================================================
// ABOUT
// ============================================================

#[component]
fn About() -> View {
    view! {
        section(class="section", id="about") {

            div(class="container") {

                div(class="section-heading split-heading"){

                    div {
                        span(class="section-kicker") {
                            "01 · WHO WE ARE"
                        }

                        h2 {
                            "More than beverages."
                        }
                    }

                    p {
                        "We build brands that live at the intersection of
                         culture, craftsmanship and changing consumer tastes."
                    }
                }


                div(class="about-grid") {

                    article(class="about-card neumorph-card") {
                        span(class="card-number") { "01" }

                        div(class="about-icon") {
                            "◌"
                        }

                        h3 {
                            "People first."
                        }

                        p {
                            "Understanding people is where every great
                             beverage brand begins."
                        }
                    }


                    article(class="about-card neumorph-card raised") {
                        span(class="card-number") { "02" }

                        div(class="about-icon pink-icon") {
                            "✦"
                        }

                        h3 {
                            "Brands with meaning."
                        }

                        p {
                            "We create distinctive brands designed to
                             become part of people's lives."
                        }
                    }


                    article(class="about-card neumorph-card") {
                        span(class="card-number") { "03" }

                        div(class="about-icon") {
                            "↗"
                        }

                        h3 {
                            "Built for tomorrow."
                        }

                        p {
                            "From new experiences to new technologies,
                             we keep looking forward."
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// BRANDS
// ============================================================

#[component]
fn Brands() -> View {
    view! {
        section(class="section brands-section", id="brands") {

            div(class="container") {

                div(class="section-heading centered"){

                    span(class="section-kicker") {
                        "02 · OUR BRANDS"
                    }

                    h2 {
                        "Different occasions."
                        br {}
                        "Different expressions."
                    }

                    p {
                        "Our portfolio brings together brands designed
                         for different tastes, occasions and generations."
                    }
                }


                div(class="brands-grid") {

                    article(class="brand-card brand-dark neumorph-card") {

                        div(class="brand-card-top") {
                            span { "01" }
                            span { "PREMIUM SPIRITS" }
                        }

                        div(class="brand-bottle bottle-gold") {
                            div(class="brand-bottle-label") {
                                "MR"
                            }
                        }

                        div(class="brand-card-bottom") {
                            h3 { "SIGNATURE" }
                            span { "Explore brand →" }
                        }
                    }


                    article(class="brand-card brand-light neumorph-card") {

                        div(class="brand-card-top") {
                            span { "02" }
                            span { "CONTEMPORARY" }
                        }

                        div(class="brand-art") {
                            div {}
                            div {}
                            div {}
                        }

                        div(class="brand-card-bottom") {
                            h3 { "AFTER DARK" }
                            span { "Explore brand →" }
                        }
                    }


                    article(class="brand-card brand-pink neumorph-card") {

                        div(class="brand-card-top") {
                            span { "03" }
                            span { "NEW GENERATION" }
                        }

                        div(class="brand-symbol") {
                            "✦"
                        }

                        div(class="brand-card-bottom") {
                            h3 { "SOCIAL" }
                            span { "Explore brand →" }
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// MARKET
// ============================================================

#[component]
fn Market() -> View {
    view! {
        section(class="section market-section", id="world") {

            div(class="container") {

                div(class="section-heading split-heading"){

                    div {
                        span(class="section-kicker") {
                            "03 · OUR WORLD"
                        }

                        h2 {
                            "A market that's moving."
                        }
                    }

                    p {
                        "India's beverage landscape is changing rapidly.
                         New consumers, new occasions and new expectations
                         are reshaping the category."
                    }
                }


                div(class="stats-grid") {

                    article(class="stat-card neumorph-card") {
                        span { "INDIA · SPIRITS" }

                        strong { "+12%" }

                        p {
                            "Volume growth in 2024"
                        }
                    }


                    article(class="stat-card neumorph-card pink-card") {
                        span { "INDIA · BEER" }

                        strong { "+38%" }

                        p {
                            "Volume growth in 2024"
                        }
                    }


                    article(class="stat-card neumorph-card") {
                        span { "INDIA · WINE" }

                        strong { "+19%" }

                        p {
                            "Volume growth in 2024"
                        }
                    }


                    article(class="stat-card neumorph-card") {
                        span { "INDIA · RTDs" }

                        strong { "+10%" }

                        p {
                            "Volume growth in 2024"
                        }
                    }
                }


                div(class="market-banner neumorph-inset") {

                    span {
                        "MARCO RICHARD"
                    }

                    strong {
                        "8%+"
                        small {
                            "INDIA MARKET SHARE"
                        }
                    }

                    span {
                        "GROWING WITH INDIA"
                    }
                }
            }
        }
    }
}


// ============================================================
// CONSUMER
// ============================================================

#[component]
fn Consumer() -> View {
    view! {
        section(class="section consumer-section") {

            div(class="container consumer-grid") {

                div(class="consumer-copy"){

                    span(class="section-kicker") {
                        "04 · UNDERSTANDING PEOPLE"
                    }

                    h2 {
                        "The consumer is changing."
                    }

                    p {
                        "The next generation of Indian consumers is
                         redefining premium beverages, social occasions
                         and brand experiences."
                    }

                    p {
                        "We believe the best brands start by listening."
                    }

                    a(
                        class="text-link",
                        href="#careers"
                    ) {
                        "MEET THE PEOPLE BEHIND THE BRANDS →"
                    }
                }


                div(class="consumer-stats") {

                    div(class="big-stat neumorph-card") {

                        strong { "40%" }

                        span {
                            "OF DEMAND IN THE LUXURY SEGMENT"
                        }
                    }


                    div(class="big-stat neumorph-card pink-card") {

                        strong { "64%" }

                        span {
                            "OF NEW GROWTH IN SINGLE MALT SALES"
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// INNOVATION
// ============================================================

#[component]
fn Innovation() -> View {
    view! {
        section(class="section", id="innovation") {

            div(class="container") {

                div(class="innovation-panel neumorph-card"){

                    div(class="innovation-copy"){

                        span(class="section-kicker") {
                            "05 · INNOVATION"
                        }

                        h2 {
                            "Tradition meets what's next."
                        }

                        p {
                            "We combine category expertise with
                             technology, data and creativity to build
                             better consumer experiences."
                        }

                        div(class="innovation-tags") {
                            span { "CONSUMER INSIGHTS" }
                            span { "DIGITAL" }
                            span { "AI" }
                            span { "INNOVATION" }
                        }
                    }


                    div(class="innovation-interface neumorph-inset"){

                        div(class="interface-header") {
                            span {}
                            span {}
                            span {}

                            b {
                                "MARCO / LAB"
                            }
                        }


                        div(class="interface-main"){

                            small {
                                "THE NEXT GENERATION OF MARKETING"
                            }

                            strong {
                                "What will you create?"
                            }

                            div(class="interface-result") {

                                span(class="ai-badge") {
                                    "AI"
                                }

                                div {
                                    b {
                                        "CONSUMER × CULTURE"
                                    }

                                    small {
                                        "Data · Creativity · Experience"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// CAREERS
// ============================================================

#[component]
fn Careers() -> View {
    view! {
        section(class="section careers-section", id="careers") {

            div(class="container") {

                div(class="careers-panel neumorph-card"){

                    div(class="careers-copy"){

                        span(class="section-kicker") {
                            "06 · CAREERS"
                        }

                        h2 {
                            "Build brands."
                            br {}
                            "Build yourself."
                        }

                        p {
                            "We're looking for curious people who want
                             to understand consumers, create ideas and
                             put them into the real world."
                        }

                        a(
                            class="soft-button primary",
                            href="#graduate-program"
                        ) {
                            "EXPLORE CAREERS"
                            span { "→" }
                        }
                    }


                    div(class="career-number") {

                        span {
                            "GRADUATE"
                        }

                        strong {
                            "2026"
                        }

                        small {
                            "MARKETING PROGRAM"
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// GRADUATE PROGRAM
// ============================================================

#[component]
fn GraduateProgram() -> View {
    view! {
        section(
            class="section graduate-section",
            id="graduate-program"
        ) {

            div(class="container") {

                div(class="section-heading centered"){

                    span(class="section-kicker") {
                        "07 · GRADUATE MARKETING PROGRAM"
                    }

                    h2 {
                        "Start your next chapter."
                    }

                    p {
                        "A hands-on opportunity to learn marketing
                         by working with real consumers, real brands
                         and real business challenges."
                    }
                }


                div(class="journey-track") {

                    article(class="journey-step neumorph-inset") {

                        span { "01" }

                        h3 {
                            "DISCOVER"
                        }

                        p {
                            "Understand consumers, categories
                             and culture."
                        }
                    }


                    div(class="journey-arrow") {
                        "→"
                    }


                    article(class="journey-step neumorph-inset") {

                        span { "02" }

                        h3 {
                            "CREATE"
                        }

                        p {
                            "Build propositions, campaigns
                             and experiences."
                        }
                    }


                    div(class="journey-arrow") {
                        "→"
                    }


                    article(class="journey-step neumorph-inset") {

                        span { "03" }

                        h3 {
                            "EXECUTE"
                        }

                        p {
                            "Take ideas into the real market."
                        }
                    }


                    div(class="journey-arrow") {
                        "→"
                    }


                    article(class="journey-step neumorph-inset") {

                        span { "04" }

                        h3 {
                            "GROW"
                        }

                        p {
                            "Turn experience into your
                             next advantage."
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// CAREER FIT
// ============================================================

#[component]
fn CareerFit() -> View {
    view! {
        section(class="section fit-section") {

            div(class="container") {

                div(class="fit-panel neumorph-card"){

                    div {

                        span(class="section-kicker") {
                            "07 · WHO WE'RE LOOKING FOR"
                        }

                        h2 {
                            "Could you be one of us?"
                        }

                        p {
                            "We value curiosity, ownership and people
                             who are excited about consumers, brands
                             and the world around them."
                        }
                    }


                    div(class="check-grid") {

                        div(class="check-item") {
                            b { "✓" }
                            span { "Fresh MBA graduate" }
                        }

                        div(class="check-item") {
                            b { "✓" }
                            span { "Marketing preferred" }
                        }

                        div(class="check-item") {
                            b { "✓" }
                            span { "Consumer curiosity" }
                        }

                        div(class="check-item") {
                            b { "✓" }
                            span { "Creative mindset" }
                        }

                        div(class="check-item") {
                            b { "✓" }
                            span { "Digital curiosity" }
                        }

                        div(class="check-item") {
                            b { "✓" }
                            span { "Ready to learn by doing" }
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// APPLICATION
// ============================================================

#[component]
fn Apply() -> View {
    view! {
        section(class="section apply-section", id="apply") {

            div(class="container") {

                div(class="apply-panel neumorph-card"){

                    div(class="apply-orb") {}

                    span(class="section-kicker") {
                        "09 · JOIN US"
                    }

                    h2 {
                        "Your next move"
                        br {}
                        em {
                            "starts here."
                        }
                    }

                    p {
                        "Marco Richard Graduate Marketing Program 2026"
                    }

                    a(
                        class="soft-button primary large",
                        href="mailto:careers@marcorichard.example"
                    ) {
                        "APPLY NOW"
                        span { "↗" }
                    }

                    small {
                        "Graduate Marketing Program · 2026"
                    }
                }
            }
        }
    }
}


// ============================================================
// CONTACT
// ============================================================

#[component]
fn Contact() -> View {
    view! {
        section(class="section contact-section", id="contact") {

            div(class="container") {

                div(class="contact-grid") {

                    div {

                        span(class="section-kicker") {
                            "06 · GET IN TOUCH"
                        }

                        h2 {
                            "Let's talk."
                        }

                        p {
                            "Whether you're interested in our brands,
                             our business or joining our team,
                             we'd love to hear from you."
                        }
                    }


                    div(class="contact-card neumorph-inset") {

                        a(
                            href="mailto:hello@marcorichard.example"
                        ) {
                            "hello@marcorichard.example"
                        }

                        //a(
                        //    href="#careers"
                        //) {
                        //    "Careers →"
                        //}

                        a(
                            href="#brands"
                        ) {
                            "Our Brands →"
                        }
                    }
                }
            }
        }
    }
}


// ============================================================
// FOOTER
// ============================================================

#[component]
fn Footer() -> View {
    view! {
        footer(class="footer") {

            div(class="container footer-inner") {

                Logo()

                div(class="footer-links") {
                    a(href="#about") { "ABOUT" }
                    a(href="#brands") { "BRANDS" }
                    a(href="#world") { "OUR WORLD" }
                    //a(href="#careers") { "CAREERS" }
                }

                span {
                    "© 2026 MARCO RICHARD"
                }
            }
        }
    }
}


// ============================================================
// APP
// ============================================================

#[component]
fn App() -> View {
    view! {
        div(class="app") {

            Navbar()

            main {

                Hero()

                About()

                Brands()

                Market()

                Consumer()

                Innovation()

                Contact()
            }

            Footer()
        }
    }
}


fn main() {
    sycamore::render(App);
}
