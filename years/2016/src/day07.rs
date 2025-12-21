use std::collections::HashSet;

#[derive(Clone)]
struct State {
    in_hypernet: bool,
    window: Vec<char>,
    abba_outside: bool,
    abba_inside: bool,
    abas: HashSet<(char, char)>,
    babs: HashSet<(char, char)>,
}

fn scan(s: &str) -> State {
    s.chars().fold(
        State {
            in_hypernet: false,
            window: Vec::new(),
            abba_outside: false,
            abba_inside: false,
            abas: HashSet::new(),
            babs: HashSet::new(),
        },
        |mut st, ch| {
            match ch {
                '[' => {
                    st.in_hypernet = true;
                    st.window.clear();
                }
                ']' => {
                    st.in_hypernet = false;
                    st.window.clear();
                }
                _ => {
                    st.window.push(ch);
                    if st.window.len() > 4 {
                        st.window.remove(0);
                    }
                    if st.window.len() == 4 {
                        let a = st.window[0];
                        let b = st.window[1];
                        let c = st.window[2];
                        let d = st.window[3];
                        if a == d && b == c && a != b {
                            if st.in_hypernet {
                                st.abba_inside = true;
                            } else {
                                st.abba_outside = true;
                            }
                        }
                    }
                    if st.window.len() >= 3 {
                        let a = st.window[st.window.len() - 3];
                        let b = st.window[st.window.len() - 2];
                        let c = st.window[st.window.len() - 1];
                        if a == c && a != b {
                            if st.in_hypernet {
                                st.babs.insert((a, b));
                            } else {
                                st.abas.insert((a, b));
                            }
                        }
                    }
                }
            }
            st
        },
    )
}

fn fp1(s: &str) -> bool {
    let st = scan(s);
    st.abba_outside && !st.abba_inside
}

fn fp2(s: &str) -> bool {
    let st = scan(s);
    st.abas.iter().any(|(a, b)| st.babs.contains(&(*b, *a)))
}

pub fn solve(input: &str) -> (String, String) {
    let p1 = input.lines().filter(|l| fp1(l)).count();
    let p2 = input.lines().filter(|l| fp2(l)).count();
    (p1.to_string(), p2.to_string())
}
