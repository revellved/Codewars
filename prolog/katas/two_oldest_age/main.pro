two_oldest_ages(Ages, [B,A]) :-
  msort(Ages,Sorted),reverse(Sorted,[A,B|_]).
  
test(1) :- two_oldest_ages([1,5,87,45,8,8], R), assertion(R == [45, 87]).
test(2) :- two_oldest_ages([6,5,83,5,3,18], R), assertion(R == [18, 83]).
