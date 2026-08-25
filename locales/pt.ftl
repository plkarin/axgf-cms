# axgf-cms — textos da interface, português.
#
# QUALIDADE AUTOMÁTICA — não revisto por uma pessoa de língua materna
# portuguesa. O vocabulário genealógico em particular («union», «affiliation»,
# «confidence») tem equivalentes estabelecidos que variam conforme a tradição
# arquivística. Correções são bem-vindas — ver CONTRIBUTING.md.
#
# REGRA: este ficheiro traduz apenas a interface. Nomes, lugares, notas e
# ofícios vêm do ficheiro .axgf e permanecem na sua própria língua e escrita.

app-name = axgf-cms

nav-tree = Árvore
nav-admin = Administração
nav-sign-in = Entrar
nav-sign-out = Sair

prefs-title = Idioma e aparência
prefs-language = Idioma
prefs-language-note = Isto altera apenas a interface. Nomes, lugares e notas são sempre mostrados na sua própria língua e escrita.
prefs-theme = Aparência
prefs-apply = Aplicar
prefs-reviewed = revisto
prefs-machine = automático, { $coverage } %

theme-light = Claro
theme-dark = Escuro
theme-system = Seguir o meu sistema
theme-high-contrast = Alto contraste
theme-sepia = Sépia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = seguro para daltonismo
theme-contrast-note = contraste máximo

tree-title-around = Em torno de { $name }
tree-title-whole = A árvore inteira
tree-lede-focused = { $ancestors ->
        [one] Um ascendente
       *[other] { $ancestors } ascendentes
    }, { $descendants ->
        [one] um descendente
       *[other] { $descendants } descendentes
    } e { $spouses ->
        [one] um parceiro
       *[other] { $spouses } parceiros
    }, { $depth } gerações para cada lado. Os mais velhos em baixo. A opacidade das linhas indica a certeza da relação — uma linha ténue é uma afirmação de que o registo não está seguro.
tree-lede-whole = Todas as pessoas do ficheiro. Os mais velhos em baixo, os mais novos em cima. A opacidade das linhas indica a certeza da relação.
tree-filter-label = Filtrar as fichas visíveis
tree-filter-placeholder = Escreva um nome…
tree-centre-on = Centrar em
tree-depth = Gerações para cada lado
tree-show = Mostrar
tree-hidden-notice = { $n ->
        [one] Uma pessoa é mostrada sem os seus dados
       *[other] { $n } pessoas são mostradas sem os seus dados
    }
tree-hidden-because-role = , porque a sua visibilidade excede o que a sua conta pode ler.
tree-hidden-because-anonymous = , porque não são públicas.
tree-hidden-sign-in = Entre se tiver uma conta.
tree-restricted-card = A ficha desta pessoa não lhe é visível
tree-empty = Este ficheiro não contém ninguém para desenhar.
tree-unplaced = Em nenhuma família registada

record-identity = Identidade
record-life-events = Acontecimentos da vida
record-family = Família
record-other-relationships = Outras relações
record-occupations = Ofícios
record-places = Lugares
record-sources-documents = Fontes e documentos
record-notes = Notas
record-history = Histórico
record-raw = Entidade em bruto
record-raw-summary-note = o JSON com que esta página foi construída
record-sources-documents-help = Cada fonte nomeia os factos desta página que nela se apoiam, ordenadas pela força da prova.
record-notes-help = Notas sobre esta ficha, incluindo texto que nenhum conversor soube interpretar e que foi conservado literalmente em vez de descartado.
record-help-toggle = O que esta secção mostra

record-gender = Género
record-living = Vivo
record-visibility = Visibilidade
record-yes = sim
record-no = não
record-name-type = Tipo de nome
record-name-used = Usado
record-name-evidence = Prova
record-transliteration = Transliteração latina
record-born = Nascido/a
record-died = Falecido/a
record-parents = Pais
record-siblings = Irmãos
record-children = Filhos
record-unknown-person = [Desconhecido]
record-restricted-person = Privado
record-restricted-title = A ficha desta pessoa não lhe é visível
record-absent-person-title = Mencionado por este ficheiro mas dele ausente
record-confidence = Certeza
record-source = Fonte
record-download = Descarregar

access-restricted-title = Não visível para si
access-restricted-anonymous = Esta ficha não é pública. Entre para ver se a sua conta a pode ler.
access-role-title = Não para o seu papel
access-role-write = A sua conta pode ler este ficheiro mas não alterá-lo. Um administrador pode elevar o seu papel a colaborador.
access-scope-title = Fora do seu ramo

error-not-found-title = Não encontrado
error-not-found-detail = Essa página não existe neste ficheiro.
error-no-such-person-title = Não existe tal pessoa
error-no-such-person-detail = Este ficheiro não contém nenhuma pessoa com esse identificador.
error-no-such-entity-title = Não existe tal entidade
error-no-such-entity-detail = Este ficheiro não contém nenhuma entidade com esse identificador.
error-deleted-while-editing = Este ficheiro não contém nenhuma entidade com esse identificador. Pode ter sido eliminada enquanto a editava.
error-no-such-file-title = Não existe tal ficheiro
error-not-an-image-title = Não é uma imagem
error-not-an-image-detail = Não há miniatura para este documento, porque não é uma imagem que esta versão saiba descodificar.
error-back = Voltar

login-title = Entrar
login-lede = As contas são criadas por um administrador.
login-username = Nome de utilizador
login-password = Palavra-passe
login-submit = Entrar
login-wrong = Esse nome de utilizador e essa palavra-passe não correspondem.
login-token-wrong = Esse testemunho não está correto.
login-throttled = Demasiadas tentativas falhadas. Aguarde alguns minutos e tente de novo.
login-no-accounts-title = Esta instalação ainda não tem contas.
login-emergency-summary = Acesso de emergência
login-emergency-label = Testemunho de emergência
login-emergency-submit = Usar o testemunho de emergência
login-sign-in-prompt = Entre para aceder ao painel de administração.

admin-title = Administração
admin-entities = Entidades
admin-create = Criar
admin-new-kind = Novo: { $kind }
admin-operations = Operações
admin-validate = Validar
admin-deduplicate = Eliminar duplicados
admin-export = Exportar o ficheiro
admin-accounts = Contas
admin-dedup-confirm = A eliminação de duplicados funde entidades e reescreve o ficheiro. Continuar?
admin-recent-changes = Alterações recentes
admin-sessions-open = { $n ->
        [one] Uma sessão aberta neste momento.
       *[other] { $n } sessões abertas neste momento.
    }
admin-no-changes-yet = Ainda nada foi alterado através desta aplicação. Cada gravação a partir de agora fica registada em { $path }.
admin-last-validation = Última validação
admin-fields = Campos
admin-raw-json = JSON em bruto
admin-save = Guardar
admin-cancel = Cancelar
admin-delete = Eliminar
admin-not-set = — não definido —
admin-edit = Editar
admin-page-of = Página { $page } de { $pages }
admin-previous = Anterior
admin-next = Seguinte
admin-saved = Guardado como versão { $version } — { $summary }
admin-not-saved = Não guardado
admin-created = Criado
admin-not-created = Não criado
admin-deleted = Eliminado
admin-not-deleted = Não eliminado — o ficheiro está inalterado
admin-what-changed = o que mudou
admin-field = Campo
admin-from = De
admin-to = Para
admin-version = versão { $version }

accounts-title = Contas
accounts-existing = Existentes
accounts-username = Nome de utilizador
accounts-role = Papel
accounts-status = Estado
accounts-branch = Ramo
accounts-last-seen = Visto pela última vez
accounts-change = Alterar
accounts-you = (você)
accounts-active = ativa
accounts-disabled = desativada
accounts-never = nunca
accounts-whole-tree = árvore inteira
accounts-roots = { $n ->
        [one] uma raiz
       *[other] { $n } raízes
    }
accounts-add = Adicionar uma conta
accounts-password-hint = Deixe em branco e uma será gerada e mostrada uma só vez. Pelo menos { $min } caracteres se a definir você.
accounts-new-password-placeholder = nova palavra-passe (em branco = manter)
accounts-email = Correio eletrónico
accounts-optional = (opcional)
accounts-create = Criar a conta
accounts-role-viewer = leitor — lê as fichas públicas e de membros
accounts-role-contributor = colaborador — além disso cria, edita e carrega
accounts-role-admin = administrador — além disso gere contas, elimina e exporta
accounts-branch-placeholder = um identificador de pessoa por linha
accounts-ids-in-bundle = Identificadores de pessoas neste ficheiro
accounts-created = { $username } criada.
accounts-updated = { $username } atualizada. Qualquer sessão que tivesse aberta foi terminada.
accounts-username-taken = Esse nome de utilizador já está ocupado.
accounts-pick-role = Escolha um papel.
accounts-no-such = Não existe tal conta.
accounts-not-saved = Não guardado: { $error }

conflict-title = Outra pessoa alterou isto antes de si
conflict-versions = Partiu da versão { $expected }; o ficheiro contém agora a versão { $current }.
conflict-both-changed = Ambos alteraram estes campos
conflict-both-changed-detail = Estes campos foram editados por ambos. O que guardar substituirá o que { $who } lá pôs:
conflict-field-by-field = Campo a campo
conflict-theirs = Para que { $who } o alterou
conflict-yours = Para que você o alterou
conflict-unchanged-by-you = não alterado por si
conflict-unchanged-by-them = não alterado por eles
conflict-what-now = E agora
conflict-reapply = Reaplicar a sua versão sobre a deles
conflict-save-over = Guardar esta sobre a deles
conflict-discard = Descartar a minha e recomeçar
conflict-their-version = A versão de { $who }, tal como o ficheiro a contém agora
conflict-history-of = Histórico desta entidade ({ $kind })

## Dates

date-unknown = Data desconhecida
date-not-recorded = Não registada
date-circa = cerca de { $date }
date-between = entre { $from } e { $to }
date-before = antes de { $date }
date-after = depois de { $date }
date-preserved = registado como «{ $text }»
date-day-month-year = { $day } de { $month } de { $year }
date-month-year = { $month } de { $year }
date-decade = os anos { $decade }
date-century = o século { $century }

month-1 = janeiro
month-2 = fevereiro
month-3 = março
month-4 = abril
month-5 = maio
month-6 = junho
month-7 = julho
month-8 = agosto
month-9 = setembro
month-10 = outubro
month-11 = novembro
month-12 = dezembro
