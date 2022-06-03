import java.util.*;
public class Crack_coding_interview{
    public static void main(String[] args){
        System.out.println("Ciao bello");
    }
    public static void sum_not_efficient(int n){
        for(int a=1;a<=n;a++){
            for(int b=1;b<=n;b++){
                for(int c=1;c<=n;c++){
                    for(int d=1;d<=n;d++){
                        if(Math.pow(a,3)+Math.pow(b,3) == Math.pow(c,3)+Math.pow(d,3)){
                            System.out.println("a== " + a + "b== " + b + "c== " + c + "d== " + d);
                            
                        }
                    }
                }
            }
        }
    }
    public static void sum_efficient(int n){
        HashMap<Integer, List> map = new HashMap<Integer, List>();
        for(int a=1;a<=n;a++){
            for(int b=1;b<=n;b++){
                int res = (int)(Math.pow(a,3) + Math.pow(b,3));
                if(map.contains(res)){
                    map.get(res).add(a);
                }
                else{ //(!map.contains(res))
                    map.put(res,new ArrayList());
                }
            }
        }



        for(int c=1;c<=n;c++){
            for(int d=1;d<=n;d++){
                /*if(Math.pow(a,3)+Math.pow(b,3) == Math.pow(c,3)+Math.pow(d,3)){
                    System.out.println("a== " + a + "b== " + b + "c== " + c + "d== " + d);
                    
                }*/
            }
        }
    }
}