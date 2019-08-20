struct Story {    title string}
// Fetches top HN stories in 8 coroutines 
fn main() {   
    resp := http.get("https://hacker-news.firebaseio.com/v0/topstories.json")?    
    ids := json.decode([]int, resp.body)?   
    mut cursor := 0    
    for _ in 0..8 { 
            go fn() {           
            for {     
                lock { // Without this lock the program will not compile          
                    if cursor >= ids.len {                   
                        break                   
                    }       
                    id := ids[cursor]               
                    cursor++              
                }               
                resp := http.get("https://hacker-news.firebaseio.com/v0/item/$id.json")?    
                story := json.decode(Story, resp.body)?           
                println(story.title)           
            }      
        }()   
    }    
    runtime.wait() // Waits for all coroutines to finish 
} 
