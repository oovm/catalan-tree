(* ::Package:: *)

treeR[1] = n;
treeR[n_] := treeR[n] = Table[f[treeR[a], treeR[n - a]], {a, 1, n - 1}]
treeC[n_] := Flatten[treeR[n] //. {f[a_List, b_] :> (f[#, b]& /@ a), f[a_, b_List] :> (f[a, #]& /@ b)}]
treeC[4] // TableForm


rank = 6
nn = Array["n{" <> ToString[# - 1] <> "}"&, rank];
oo = Array["f{" <> ToString[# - 1] <> "}"&, rank - 1];
ff = ReplacePart[#, Thread[Position[#, n] -> nn] ~ Join ~ Thread[Position[#, f] -> oo]]&
fn = "|f, n|" <> StringReplace[ToString@#, {"[" -> "(", "]" -> ")", "{" -> "[", "}" -> "]"}]&;
fc = fn /@ Evaluate[ff /@ treeC[rank]]
TemplateApply[
    "let fs: &[fn(&[fn(T, T) -> T], &[T]) -> T; `1`] = &[`2`];",
    {
        Length@fc,
        StringRiffle[fc, ","]
    }
]
% // CopyToClipboard

