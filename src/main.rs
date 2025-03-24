use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    id: String,
    name: String,
    category: String,
}

struct SearchSystem {
    index: HashMap<String, Vec<Product>>, // Mapeia palavras-chave para produtos
}

impl SearchSystem {
    fn new() -> Self {
        SearchSystem {
            index: HashMap::new(),
        }
    }

    fn add_product(&mut self, product: Product, keywords: Vec<&str>) {
        for keyword in keywords {
            self.index
                .entry(keyword.to_lowercase()) // Usa minúsculas para evitar problemas
                .or_insert(Vec::new())
                .push(product.clone());
        }
    }

    fn search(&self, query: &str) -> Vec<Product> {
        self.index.get(&query.to_lowercase()).cloned().unwrap_or_else(Vec::new)
    }
}

fn main() {
    let mut search_system = SearchSystem::new();

    // Adicionando produtos ao índice
    search_system.add_product(Product {
        id: "P001".to_string(),
        name: "Notebook Dell XPS".to_string(),
        category: "Eletrônicos".to_string(),
    }, vec!["notebook", "laptop", "computador"]);

    search_system.add_product(Product {
        id: "P002".to_string(),
        name: "Smartphone Samsung Galaxy".to_string(),
        category: "Celulares".to_string(),
    }, vec!["smartphone", "celular", "samsung"]);

    search_system.add_product(Product {
        id: "P003".to_string(),
        name: "Fone de Ouvido Bluetooth".to_string(),
        category: "Acessórios".to_string(),
    }, vec!["fone", "bluetooth", "acessório"]);

    // Simulando uma busca
    let query = "notebook";
    let results = search_system.search(query);

    if results.is_empty() {
        println!("Nenhum produto encontrado.");
    } else {
        println!("Produtos encontrados para '{}':", query);
        for product in results {
            println!("ID: {}, Nome: {}, Categoria: {}", product.id, product.name, product.category);
        }
    }
}

