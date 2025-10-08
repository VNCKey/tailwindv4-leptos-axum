use leptos::prelude::*;


// ========================================
// 📦 Componente: Header
// 🔹 Ubicación: /components/header.rs
// 🔹 Función: Muestra logo y menú principal
// 🔹 Autor: Luis Alexander
// ========================================

#[component]
pub fn Header() -> impl IntoView {
	view! {
		<header id="header" data-scope="header" class="header">
			<nav class="container1 flex">
				// * Logo VNCkey
				// FIXME:
				<div class="logo flex">
					<div class="logo_img">
						<img src="/logo.png" alt="" />
					</div>
					<a href="/">
						<h1>Blog</h1>
					</a>
				</div>
				<div class="nav_list">
					<ul class="flex">
						<li>
							<a href="/">Home</a>
						</li>
						<li>
							<a href="/">
								<span>About Me</span>
								<i class="ri-arrow-down-s-line"></i>
							</a>
						</li>
						<li>
							<a href="/">
								<span>Projects</span>
								<i class="ri-arrow-down-s-line"></i>
							</a>
						</li>
						<li>
							<a href="/">
								<span>Blog</span>
								<i class="ri-arrow-down-s-line"></i>
							</a>
						</li>
						<li>
							<a href="/">
								<span>Page</span>
								<i class="ri-arrow-down-s-line"></i>
							</a>
						</li>
						<li>
							<a href="/">
								<span>Contact</span>
								<i class="ri-arrow-down-s-line"></i>
							</a>
						</li>
					</ul>
				</div>
				// * Buscar
				<div class="search-dark-sub flex">
					<div class="search-bar">
						<i class="ri-search-eye-line"></i>

						// * Contenedor del formulario
						<div class="search-click">
							// * Formulario
							<form action="#" role="search">
								<input
									type="text"
									class="form-control"
									placeholder="Buscar artículos"
								/>
								<i class="ri-search-line"></i>
							</form>

							<div class="tag-sec">
								<h3>Temas populares</h3>
								<div class="tags">
									<a href="/">#Rust <span>,</span></a>
									<a href="/">#APIs <span>,</span></a>
									<a href="/">#MobileDev <span>,</span></a>
									<a href="/">#FullStack <span>,</span></a>
									<a href="/">#WebAssembly</a>
								</div>
							</div>
						</div>
					</div>
					<div class="toggle-switch">
						<input type="checkbox" class="toggle-checkbox" id="dark-mode-toggle" />
						<label for="dark-mode-toggle" class="toggle-label"></label>
						<button>
							<a href="">Subcribe</a>
						</button>
					</div>
				</div>

			</nav>
		</header>
	}
}

