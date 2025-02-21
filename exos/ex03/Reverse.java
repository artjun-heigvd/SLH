public class Reverse {

    public static void main(String[] args) {

        String toReverse = "Hello world!";

        System.out.println(reverse(toReverse));

    }

    static String reverse(String reverse) {

        String res = "";

        for(int i = reverse.length(); i > 0; --i){
            res += reverse.charAt(i - 1);
        }

        return res;
    }
}