use std::error::Error; 
use std::fs::File; 
use std::io; 
use std::path::Path; 
use zip::ZipArchive; 


fn  main () ->  Result <(), Box < dyn Error>> { 

    let zip_file_path = Path::new("compressed_files.zip");

    let zip_file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    let path = Path::new( "extracted_files" );

    // Создать каталог, если он не существует. 
    if !path.exists() {
        std::fs::create_dir(path)?;
    } 

    // Перебрать файлы в ZIP-архиве. zip_img
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();

        // Создаем путь к извлеченному файлу в целевом каталоге. 
        let target_path = extraction_dir.join(file_name);
        // Создаем целевой каталог, если он не существует. 
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        } 

        let mut output_file = File::create(&target_path)?;
        // Читаем содержимое файла из ZIP-архива и записываем его в целевой файл.
        io::copy(& mut file, & mut output_file)?;
    } 

    println! ( "Файлы успешно извлечены в {:?}" , extraction_dir); 

    Ok (()) 
}


 fn main() {
    let mut new = 0;
    let v = vec!["1_a.jpg","1_a.png","1_a.jpg","1_a.jpg","1_a.png"];
    for x in &v {
        let n: Vec<&str> = x.split(".").collect();
        new += 1;
        let new_name = format!("{}.{}", new, n[1]);
        if new == 3{
            break;
        }
        println!(" new_name..! {:?}", new_name);
    }
 }


 file_name..! "zip_img/1_a.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/1_a.jpg"
 file_name..! "zip_img/1_b.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/1_b.jpg"
 file_name..! "zip_img/1_list.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/1_list.jpg"
 file_name..! "zip_img/2_a.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/2_a.jpg"
 file_name..! "zip_img/2_list.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/2_list.jpg"
 file_name..! "zip_img/3_a.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/3_a.jpg"
 file_name..! "zip_img/4_b.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/4_b.jpg"
 file_name..! "zip_img/full_1.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/full_1.jpg"
 file_name..! "zip_img/full_2.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/full_2.jpg"
 file_name..! "zip_img/header_600.jpg"
 file_path..! "./static/assets/photo/slider/two@example.com/zip_img/header_600.jpg"
 dir..! "./static/assets/photo/zip/two@example.com/"