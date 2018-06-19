Require Import Arith.
Require Import List.

(* This is how you might evaluate an expression. *)
Compute 1 + 1.

(* This is how you might check the type of an expression or value. *)
Check 1 + 1.

(* This is an example of a function definition. *)
Definition plusOne(n : nat) := n + 1.

Check plusOne(0).

(* This is an example of a pattern matching statement. *)
Definition isZero(n : nat) :=
  match n with
    0 => true
  | _ => false
end.

Compute isZero(0).
Compute isZero(1).

(*
    This is example of a recursive function in Coq.
*)
Fixpoint add n m : nat :=
  match n with
    0 => m
  | S p => add p (S m)
end.

(* This is how we would call the function defined above. *)
Compute add 5 6.
