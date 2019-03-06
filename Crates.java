class Crates {
    private static native String resolve(String req, String versions);


    static {
        System.loadLibrary("semver_jni_rs");
    }

    public static void main(String[] args) {

        String output = Crates.resolve(">= 1.0.0", "0.0.0,1.1.1-beta.1,1.1.1-beta.2");
        System.out.println(output);
      
    }

}
