macro_rules! pop {
  ($s:ident) => {{
    $s.pop()
      .ok_or_else(|| format!("FeelsDankMan something went wrong..."))?
  }};
}

macro_rules! pop2 {
  ($s:ident) => {{
    let r = pop!($s);
    let l = pop!($s);

    (l, r)
  }};
}
