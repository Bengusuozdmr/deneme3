fn main() {
    let sonuc= topla(5, 15);
    let sonuc1:i32= cıkar(55, 30);
    let sonuc2:i32= carpma(42, 58);
    let sonuc3:i32= bolme(80, 5);
    let sonuc4:i32= dikdortgenin_alani(10, 5);
    let sonuc5:i32=usluSayi(12, 12);
    println!("{}",sonuc5);
    
    
    
    println!("{}",sonuc4);
    println!("{}",sonuc3);
    println!("{}",sonuc2);
    println!("{}",sonuc1);
    println!("{}", sonuc);
}

fn topla(a:i32, b:i32) -> i32{
    a +b
}
fn cıkar(c:i32, d:i32) ->i32{
     c-d
}
fn  carpma(e:i32, f:i32)-> i32{
     e*f
}
fn bolme(g:i32, h:i32)-> i32{
    g/h
}
fn dikdortgenin_alani( uzunluk: i32, genislik: i32)->i32{
    uzunluk*genislik
}
fn usluSayi(n:i32, n1:i32)-> i32{
    n*n1
}
 



// 4 işlem
// dikdörtgenin alanı
// üslü sayı
 



