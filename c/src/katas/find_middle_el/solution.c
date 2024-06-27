int gimme(const int tr[3])
{
  if ((tr[0] < tr[1] && tr[1] < tr[2]) || (tr[2] < tr[1] && tr[1] < tr[0])) return 1;
  else if ((tr[1] < tr[0] && tr[0] < tr[2]) || (tr[2] < tr[0] && tr[0] < tr[1])) return 0;
  else return 2;
}
