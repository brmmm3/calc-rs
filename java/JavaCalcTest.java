class JavaCalcTest {
    private static native long calcNew();
    private static native double calcAdd(long calc_ptr, double value);
    private static native double calcSub(long calc_ptr, double value);
    private static native double calcResult(long calc_ptr);

    static {
        System.loadLibrary("javacalclib");
    }

    public static void main(String[] args) {
        long calc_ptr = calcNew();
        System.out.println("Add " + calcAdd(calc_ptr, 1.0));
        System.out.println("Sub " + calcSub(calc_ptr, 1.5));
        System.out.println("Result " + calcResult(calc_ptr));
    }
}