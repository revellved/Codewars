check_exam([],[],Score) :- Score = 0.
check_exam([''|T], [_|T2],Score) :- check_exam(T,T2, Sc), Score is Sc.
check_exam([H|T], [H|T2], Score) :- check_exam(T, T2, Sc), Score is Sc + 4.
check_exam([_|T], [_|T2], Score) :- check_exam(T, T2, Sc), Score is Sc - 1.

test(fixed_tests, forall(member((Array1, Array2, Expected), [
    ([a, a, b, b], [a, c, b, d], 6),
    ([a, a, c, b], [a, a, b, ''], 7),
    ([a, a, b, c], [a, a, b, c], 16),
    ([b, c, b, a], ['', a, a, c], 0)
]))) :-
    check_exam(Array1, Array2, Actual),
    assertion(Actual == Expected).
