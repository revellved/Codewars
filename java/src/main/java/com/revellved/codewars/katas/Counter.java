package com.revellved.codewars.katas;

import java.util.*;
import java.util.function.*;
import java.util.stream.*;

@SuppressWarnings("serial")
public class Counter<T> extends HashMap<T, Long> {
  private static final Long INITIAL_VALUE = 1L;
  private static final Long NOT_KEY_VALUE = 0L;

  public Counter() {
    super();
  }

  public Counter(int capacity) {
    super(capacity);
  }

  public Counter(int capacity, float loadFactor) {
    super(capacity, loadFactor);
  }

  public Counter(Map<? extends T, Long> map) {
    super(map);
  }

  public Counter(T[] arr) {
    pushAll(arr);
  }

  public Counter(Collection<? extends T> coll) {
    pushAll(coll);
  }

  public Counter(Stream<? extends T> stream) {
    pushAll(stream);
  }

  private static <T> Counter<T> filledBy(Consumer<Counter<T>> pusher) {
    Counter<T> result = new Counter<>();
    pusher.accept(result);
    return result;
  }

  public static Counter<Boolean> of(boolean... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Byte> of(byte... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Character> of(char... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Double> of(double... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Float> of(float... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Integer> of(int... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Long> of(long... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<Short> of(short... arr) {
    return filledBy(c -> pushAll(c, arr));
  }

  public static Counter<String> of(String s) {
    return filledBy(c -> pushAll(c, s));
  }

  @Override
  public Long get(Object key) {
    return getOrDefault(key, NOT_KEY_VALUE);
  }

  Stream<T> elements() {
    return entrySet().stream().filter(e -> e.getValue() > 0L)
        .flatMap(e -> LongStream.range(0L, e.getValue()).mapToObj(i -> e.getKey()));
  }

  List<T> elementsAsList() {
    return elements().collect(Collectors.toList());
  }

  Stream<Entry<T, Long>> mostCommon() {
    return entrySet().stream()
        .sorted(Comparator.<Entry<T, Long>>comparingLong(Entry::getValue).reversed());
  }

  List<Entry<T, Long>> mostCommonAsList() {
    return mostCommon().collect(Collectors.toList());
  }

  Stream<Entry<T, Long>> mostCommon(int n) {
    return mostCommon().limit(n);
  }

  List<Entry<T, Long>> mostCommonAsList(int n) {
    return mostCommon(n).collect(Collectors.toList());
  }

  public void push(T key) {
    merge(key, INITIAL_VALUE, Long::sum);
  }

  public void push(T key, long n) {
    merge(key, n, Long::sum);
  }

  public void pushAll(Collection<? extends T> coll) {
    for (T t : coll)
      push(t);
  }

  public void pushAll(T[] arr) {
    for (T t : arr)
      push(t);
  }

  public void pushAll(Stream<? extends T> stream) {
    stream.forEach(t -> push(t));
  }

  public void pushAll(Map<? extends T, Long> other) {
    other.entrySet().forEach(e -> merge(e.getKey(), e.getValue(), Long::sum));
  }

  public static void pushAll(Counter<Boolean> cnt, boolean... arr) {
    for (boolean val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Byte> cnt, byte... arr) {
    for (byte val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Character> cnt, char... arr) {
    for (char val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Double> cnt, double... arr) {
    for (double val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Float> cnt, float... arr) {
    for (float val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Integer> cnt, int... arr) {
    for (int val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Long> cnt, long... arr) {
    for (long val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<Short> cnt, short... arr) {
    for (short val : arr)
      cnt.push(val);
  }

  public static void pushAll(Counter<String> cnt, String s) {
    for (int i = 0, len = s.length(); i < len; i++)
      cnt.push(String.valueOf(s.charAt(i)));
  }

  private Counter<T> multisetOp(Counter<? extends T> other, LongBinaryOperator op) {
    Counter<T> result = new Counter<T>(this);
    Iterator<Entry<T, Long>> it = result.entrySet().iterator();
    while (it.hasNext()) {
      Entry<T, Long> e = it.next();
      if (!other.containsKey(e.getKey())) {
        long c = op.applyAsLong(e.getValue(), 0L);
        if (c > 0L)
          e.setValue(c);
        else
          it.remove();
      }
    }
    other.entrySet().forEach(e -> result.compute(e.getKey(), (key, count) -> {
      if (count == null)
        count = NOT_KEY_VALUE;
      long c = op.applyAsLong(count, e.getValue());
      return c > 0L ? c : null;
    }));
    return result;
  }

  public Counter<T> add(Counter<? extends T> other) {
    return multisetOp(other, Long::sum);
  }

  public Counter<T> sub(Counter<? extends T> other) {
    return multisetOp(other, (x, y) -> x - y);
  }

  public Counter<T> intersect(Counter<? extends T> other) {
    return multisetOp(other, Long::min);
  }

  public Counter<T> union(Counter<? extends T> other) {
    return multisetOp(other, Long::max);
  }

  public Counter<T> subtract(Counter<? extends T> other) {
    Counter<T> result = new Counter<T>(this);
    other.entrySet().forEach(e -> result.merge(e.getKey(), -e.getValue(), Long::sum));
    return result;
  }

  public Counter<T> mul(long n) {
    Counter<T> result = new Counter<T>(this);
    result.replaceAll((key, count) -> count * n);
    return result;
  }

  @Override
  public String toString() {
    return String.format("Counter(%s)", super.toString());
  }
}
