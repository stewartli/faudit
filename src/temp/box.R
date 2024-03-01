#' @description "Welcome test"
#' @export
hello <- function(...) {
    cat(cli::col_red("✔ Welcome to FAudit\n"))
}

#' @description "Check if the required R packages are installed""
#' @export
check_r_pkg <- function() {
    deps <- c(
        "DBI", "data.table", "tidyverse", "pointblank", "gt", "shiny", "quarto", "renv", "target",
        "devtools", "usethis", "fusen", "pak", "cli"
    )
    needed <- setdiff(deps, rownames(utils::installed.packages()))
    if (length(needed) == 0) {
        cat("✔ you are good now")
    } else {
        answer <- utils::askYesNo("Do you want to install all?")
        if (!is.na(answer) && answer) {
            utils::install.packages(needed)
        } else {
            cat("✘ you need to install missing packages")
        }
    }
}

#' @description "Check if the required Python packages are installed"
#' @export
# def check_py_pkg():
#     pass

#' @description "Create RStudio project"
#' @export
create_rsproj <- function() {
    suppressMessages(usethis::create_project(path = here::here(), rstudio = TRUE, open = FALSE))
}

#' @description "Create a table out of config.json"
#' @export
show_activity <- function() {
    jsonlite::fromJSON(file.path(Sys.getenv("USER_FA_DIR"), "/config.json")) %>%
        as.data.frame() %>%
        dplyr::rename_with(~ gsub("[a-zA-Z]+?\\.([a-zA-Z]+?)", "\\1", .x))
}
