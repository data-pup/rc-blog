(*
    This is am examplt of defining theorems in Coq.
*)

Theorem tautology : forall A: Prop, A -> A.
Proof.
intros A.
intros Hyp.
exact Hyp.
Qed.

Theorem modusPonens:
    forall A B: Prop, (A /\ (A->B)) -> B.
Proof.
intros A B H.
destruct H.
apply H0.
exact H.
Qed.
