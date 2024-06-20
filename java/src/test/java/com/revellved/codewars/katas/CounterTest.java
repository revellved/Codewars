package com.revellved.codewars.katas;

import static org.junit.Assert.*;

import java.util.*;
import java.util.stream.*;
import java.math.BigInteger;

import org.junit.Test;

public class CounterTest {
  @Test
  public void test_CONSTRUCTORS_Basics() {
    Counter<Integer> c = null;

    try {
      c = new Counter<>();
    } catch (Exception e) {
      System.out.println(e);
      fail("Failed trying to use basic constructor:\n Counter<Integer> c = new Counter() (trace displayed above)");
    }
    try {
      c = new Counter<>(100);
    } catch (Exception e) {
      System.out.println(e);
      fail("Failed trying to use basic constructor:\n Counter<Integer> c = new Counter(100) (trace displayed above)");
    }
    try {
      c = new Counter<>(100, .75f);
    } catch (Exception e) {
      System.out.println(e);
      fail(
          "Failed trying to use basic constructor:\n Counter<Integer> c = new Counter(100, .75f) (trace displayed above)");
    }
  }

  @Test
  public void test_CONSTRUCTORS_CountOneShot() {

    // COLLECTIONS:

    List<Integer> lst = Arrays.asList(-8, 2, 3, 5, 6, 89, -8, -8, 3, 53, 6, 3, 5, 8);

    Map<Integer, Long> expectedLst = new HashMap<Integer, Long>() {
      {
        put(-8, 3L);
        put(2, 1L);
        put(3, 3L);
        put(5, 2L);
        put(6, 2L);
        put(89, 1L);
        put(53, 1L);
        put(8, 1L);
      }
    };

    System.out.println(new Counter<Integer>(new ArrayList<Integer>(lst)) == expectedLst);
    assertEquals(expectedLst, new Counter<Integer>(new ArrayList<Integer>(lst)));
    assertEquals(expectedLst, new Counter<Integer>(new LinkedList<Integer>(lst)));

    Map<Integer, Long> expectedSet = new HashMap<Integer, Long>() {
      {
        expectedLst.forEach((k, v) -> put(k, 1L));
      }
    };
    assertEquals(expectedSet, new Counter<Integer>(new HashSet<Integer>(lst)));

    // ARRAYS:

    String[] arr = { "a", "a", "a", "b", "b", "c", "c", "c", "c", "c", "c" };
    Map<String, Long> expected1 = new HashMap<String, Long>() {
      {
        put("a", 3L);
        put("b", 2L);
        put("c", 6L);
      }
    };
    assertEquals(expected1, new Counter<String>(arr));

    BigInteger _0 = BigInteger.ZERO,
        _1 = BigInteger.ONE,
        _2 = new BigInteger("2");
    BigInteger[] bArr = { _0, _0, _0, _1, _2, _0, _0, _1, _2, _0, _2 };
    Map<BigInteger, Long> expected2 = new HashMap<BigInteger, Long>() {
      {
        put(_0, 6L);
        put(_1, 2L);
        put(_2, 3L);
      }
    };
    assertEquals(expected2, new Counter<BigInteger>(bArr));

    // STREAMS:

    assertEquals(expectedLst, new Counter<Integer>(lst.stream()));
    assertEquals(expected2, new Counter<BigInteger>(Arrays.stream(bArr)));

    // MAP:

    Map<String, Long> map = new HashMap<String, Long>() {
      {
        put("a", 3L);
        put("b", 2L);
        put("c", 1L);
        put("helloww", -2L);
      }
    };
    assertEquals(map, new Counter<String>(map));
  }

  @Test
  public void test_STATIC_BUILDERS_ForPrimitives() {

    assertEquals(new HashMap<Boolean, Long>() {
      {
        put(true, 2L);
        put(false, 1L);
      }
    },
        Counter.of(new boolean[] { true, false, true }));

    assertEquals(new HashMap<Byte, Long>() {
      {
        put((byte) 0, 2L);
      }
    },
        Counter.of(new byte[] { 0, 0 }));

    assertEquals(new HashMap<Float, Long>() {
      {
        put(0f, 1L);
        put(2.67f, 3L);
      }
    },
        Counter.of(new float[] { 2.67f, 0f, 2.67f, 2.67f }));

    assertEquals(new HashMap<Short, Long>() {
      {
        put((short) 0, 2L);
      }
    },
        Counter.of(new short[] { 0, 0 }));

    assertEquals(new HashMap<Double, Long>() {
      {
        put(0d, 1L);
        put(2.67, 3L);
      }
    },
        Counter.of(new double[] { 2.67, 0d, 2.67, 2.67 }));

    Map<Integer, Long> expectedLst = new HashMap<Integer, Long>() {
      {
        put(-8, 3L);
        put(2, 1L);
        put(3, 3L);
        put(5, 2L);
        put(6, 2L);
        put(89, 1L);
        put(53, 1L);
        put(8, 1L);
      }
    };
    assertEquals(expectedLst,
        Counter.of(new int[] { -8, 2, 3, 5, 6, 89, -8, -8, 3, 53, 6, 3, 5, 8 }));

    assertEquals(new HashMap<Long, Long>() {
      {
        put(0L, 2L);
      }
    },
        Counter.of(new long[] { 0L, 0L }));

    char[] arr = { 'a', 'a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'c', 'c' };
    Map<Character, Long> expected2 = new HashMap<Character, Long>() {
      {
        put('a', 3L);
        put('b', 2L);
        put('c', 6L);
      }
    };
    assertEquals(expected2, Counter.of(arr));

    Map<String, Long> expected1 = new HashMap<String, Long>() {
      {
        put("a", 3L);
        put("b", 2L);
        put("c", 6L);
      }
    };
    assertEquals(expected1, Counter.of("aacabcbcccc"));

  }

  @Test
  public void test_OBSERVERS_get_and_toString() {
    Counter<BigInteger> cnt = new Counter<>(Arrays.asList(BigInteger.ONE,
        BigInteger.ONE, BigInteger.ONE));

    String msg = "Default value of 0L should be automatically returned with the 'get' method (Override)";
    BigInteger newKey = new BigInteger("102554");
    try {
      assertEquals(msg, Long.valueOf("0"), cnt.get(newKey));
    } catch (Exception e) {
      fail(msg + ": " + e);
    }
    assertTrue("Using get with a non existant key shouldn't create that key",
        !cnt.containsKey(newKey));

    assertEquals("Counter({a=5})", Counter.of("aaaaa").toString());
    assertEquals("Counter({1=3})", cnt.toString());
  }

  @Test
  public void test_PUSH_CountStringsOnYourFingers() {
    Counter<String> c = new Counter<String>();
    c.push("a");
    c.push("a");
    c.push("a");
    c.push("b");
    c.push("b");
    c.push("c");
    assertEquals(new HashMap<String, Long>() {
      {
        put("a", 3L);
        put("b", 2L);
        put("c", 1L);
      }
    }, c);
    c.push("helloww", -2);
    assertEquals(new HashMap<String, Long>() {
      {
        put("a", 3L);
        put("b", 2L);
        put("c", 1L);
        put("helloww", -2L);
      }
    }, c);
  }

  @Test
  public void test_PUSH_CountIntsOnYourFingers() {
    Counter<Integer> c = new Counter<Integer>();
    c.push(5);
    c.push(6);
    c.push(7);
    c.push(5);
    assertEquals(new HashMap<Integer, Long>() {
      {
        put(5, 2L);
        put(6, 1L);
        put(7, 1L);
      }
    }, c);
    c.push(22, -4);
    assertEquals(new HashMap<Integer, Long>() {
      {
        put(5, 2L);
        put(6, 1L);
        put(7, 1L);
        put(22, -4L);
      }
    }, c);
  }

  @Test
  public void test_PUSH_CountListsOnYourFingers() {
    List<Double> a = Arrays.asList(1.2d, 5.3d, 5.3d);
    List<Double> b = Arrays.asList(5.3d, 1.2d, 5.3d);
    List<Double> c = Arrays.asList(5.1d, 4.0d, 6.5d, 6.9d);
    Counter<List<Double>> cnt = new Counter<List<Double>>();
    cnt.push(a);
    cnt.push(a);
    cnt.push(a);
    cnt.push(b);
    cnt.push(a);
    cnt.push(c);
    cnt.push(c);
    assertEquals(new HashMap<List<Double>, Long>() {
      {
        put(a, 4L);
        put(b, 1L);
        put(c, 2L);
      }
    }, cnt);
    cnt.push(c, -2);
    assertEquals(new HashMap<List<Double>, Long>() {
      {
        put(a, 4L);
        put(b, 1L);
        put(c, 0L);
      }
    }, cnt);
  }

  @Test
  public void test_MATHS_multisets_and_subtract() {

    Counter<String> a = Counter.of("aaab"),
        b = Counter.of("abb"),
        a_add_b = Counter.of("aaaabbb"),
        a_sub_b = Counter.of("aa"),
        b_sub_a = Counter.of("b"),
        a_inter_b = Counter.of("ab"),
        a_union_b = Counter.of("aaabb"),
        a_subtract_b = new Counter<>(new HashMap<String, Long>() {
          {
            put("a", 2L);
            put("b", -1L);
          }
        }),
        b_subtract_a = new Counter<>(new HashMap<String, Long>() {
          {
            put("a", -2L);
            put("b", 1L);
          }
        });

    String msg = "%s.%s(%s)", func = "", errMsg = "";

    func = "add";
    errMsg = String.format(msg, a, func, b);
    assertEquals(errMsg, a_add_b, a.add(b));

    errMsg = func + " should be Symmetric: " + String.format(msg, b, func, a);
    assertEquals(errMsg, a_add_b, b.add(a));

    func = "intersect";
    errMsg = String.format(msg, a, func, b);
    assertEquals(errMsg, a_inter_b, a.intersect(b));

    errMsg = func + " should be Symmetric: " + String.format(msg, b, func, a);
    assertEquals(errMsg, a_inter_b, b.intersect(a));

    func = "union";
    errMsg = String.format(msg, a, func, b);
    assertEquals(errMsg, a_union_b, a.union(b));

    errMsg = func + " should be Symmetric: " + String.format(msg, b, func, a);
    assertEquals(errMsg, a_union_b, b.union(a));

    func = "sub";
    errMsg = String.format(msg, a, func, b);
    assertEquals(errMsg, a_sub_b, a.sub(b));

    errMsg = func + " shouldn't be Symmetric: " + String.format(msg, b, func, a);
    assertEquals(errMsg, b_sub_a, b.sub(a));

    func = "subtract";
    errMsg = String.format(msg, a, func, b);
    assertEquals(errMsg, a_subtract_b, a.subtract(b));

    errMsg = func + " shouldn't be Symmetric: " + String.format(msg, b, func, a);
    assertEquals(errMsg, b_subtract_a, b.subtract(a));
  }

  /****************************
   * 
   * INHERITANCE TESTS
   *************************/

  private static class Person {

    private final String name;

    public Person(String name) {
      this.name = name;
    }

    public String getName() {
      return name;
    }

    @Override
    public boolean equals(Object obj) {
      return (obj instanceof Person) && (Objects.equals(name, ((Person) obj).name));
    }

    @Override
    public int hashCode() {
      return Objects.hashCode(name);
    }
  }

  private static class President extends Person {
    public President(String name) {
      super(name);
    }
  }

  private static class CodeWarrior extends Person {
    public CodeWarrior(String name) {
      super(name);
    }
  }

}
