# axgf-cms — textos da interface, português.
#
# QUALIDADE AUTOMÁTICA — não revisto por uma pessoa de língua materna
# portuguesa. O vocabulário genealógico tem equivalências firmadas que variam
# conforme a tradição arquivística, e esta tradução pode estar errada.
# Correcções são bem-vindas — ver CONTRIBUTING.md.
#
# Variedade: português europeu.
#
# Escolhas adoptadas (contestáveis):
#   union → união · link → vínculo · confidence → certeza
#   reliability → fiabilidade · source → fonte
#   primary source → fonte primária · occupation → ocupação
#   record → ficha · archive → arquivo · godparent → padrinho/madrinha
#   witness → testemunha · speculative → hipotético
#
# Plural: regras CLDR one / other. Nunca substituir por lógica própria.
#
# Datas: «12 de abril de 1923» — com as duas preposições e o mês em minúscula.
# A tabela dos meses vive dentro do próprio padrão da data.
#
# REGRA: este ficheiro traduz apenas a interface. Nomes, lugares, notas e
# ofícios vêm do arquivo familiar e ficam na sua língua e na sua escrita.

app-name = ax-genealogy

## Cabeçalho e rodapé

nav-tree = Árvore
nav-convert = Importar
nav-admin = Administração
nav-sign-in = Entrar
nav-sign-out = Sair
footer-open-format = O arquivo da sua família é um único ficheiro que fica consigo, escrito num formato aberto: continuará a abrir muito depois de este sítio ter desaparecido.
footer-open-format-link = Sobre o formato

## Preferências

prefs-title = Língua e aspecto
prefs-language = Língua
prefs-theme = Aspecto
prefs-background = Fundo
prefs-background-on = Um véu suave de cor por trás da página
prefs-apply = Aplicar
prefs-reviewed = revista
prefs-machine = automática, { $coverage } %
prefs-machine-complete = completa, ainda por rever
prefs-machine-title = Traduzida sem revisão por uma pessoa de língua materna. O vocabulário genealógico em particular pode estar errado: as palavras para uma união, um padrinho ou uma fonte primária mudam conforme a tradição arquivística de cada país. Correcções são bem-vindas, e o CONTRIBUTING.md diz por onde começar.

theme-light = Claro
theme-dark = Escuro
theme-system = Como o sistema
theme-high-contrast = Contraste elevado
theme-sepia = Sépia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = seguro para daltonismo
theme-contrast-note = contraste máximo

## Árvore

tree-title-around = À volta de { $name }
tree-title-whole = A árvore inteira
tree-lede-focused = { $ancestors ->
        [one] Um antepassado
       *[other] { $ancestors } antepassados
    }, { $descendants ->
        [one] um descendente
       *[other] { $descendants } descendentes
    } e { $spouses ->
        [one] um parceiro
       *[other] { $spouses } parceiros
    }, { $depth } gerações para cada lado.
tree-filter-label = Filtrar as fichas visíveis
tree-filter-placeholder = Escreva um nome…
tree-centre-on = Centrar em
tree-depth = Gerações para cada lado
tree-show = Mostrar
tree-hidden-notice = { $n ->
        [one] Uma pessoa é mostrada sem os seus dados
       *[other] { $n } pessoas são mostradas sem os seus dados
    }
tree-hidden-because-role = , porque a sua visibilidade está acima do que a sua conta pode ler.
tree-hidden-because-anonymous = , porque não são públicas.
tree-hidden-sign-in = Entre, se tiver conta.
tree-restricted-card = Esta ficha não lhe é visível
tree-empty = Ainda não há ninguém para desenhar.
tree-unplaced = Em nenhuma família registada

## A ficha

record-identity = Identidade
record-life-events = Factos da vida
record-family = Família
record-other-relationships = Outras relações
record-occupations = Ocupações
record-places = Lugares
record-sources-documents = Fontes e documentos
record-notes = Notas
record-history = Histórico de alterações
record-raw = Dados em bruto
record-raw-summary-note = o JSON com que esta página foi construída

record-identity-help = Cada nome registado com o seu tipo, o período em que foi usado e a fonte que o sustenta, com a escrita própria ao lado da transliteração latina onde diferem, além do género, se está vivo e a visibilidade.
record-life-events-help = Nascimento, óbito e cada facto em que esta pessoa tomou parte, por data, cada um com o seu papel — assim um casamento de que foi apenas testemunha aparece ao lado do seu próprio. Um dado sem data vai para o fim, em vez de fingir que vem primeiro.
record-family-help = Pais e irmãos, depois cada união com o seu tipo, as datas, o lugar, como terminou e os filhos por ordem de nascimento.
record-other-relationships-help = Cada vínculo com esta pessoa numa das pontas, lido do seu lado: a mesma ficha aparece como «padrinho de» de uma ponta e «afilhado de» da outra.
record-occupations-help = As ocupações como períodos sobre um mesmo eixo, de modo que dois empregos se comparem a olho; onde falta um extremo a barra fica aberta.
record-places-help = Cada lugar que esta ficha toca, com o que ali aconteceu e com a história das fronteiras que faz um lugar ter sentido ao longo do tempo.
record-sources-documents-help = Cada fonte nomeia os factos desta página que nela se apoiam, por ordem da força da prova.
record-notes-help = Notas sobre esta ficha, incluindo o texto que nenhum conversor soube interpretar e que foi guardado à letra em vez de descartado.
record-history-help = Cada alteração guardada desta ficha, a mais recente primeiro. Quem corrigiu o quê é um facto sobre as pessoas que mantêm a árvore, não sobre a família que nela está: por isso fica fora do arquivo exportado e só é mostrado aos parentes com sessão iniciada.
record-raw-help = Aqui não há nada gerado para mostrar: esta é a ficha exactamente como está guardada, até aos nomes dos campos. Se um dia tiver de ler o arquivo sem este sítio, é isto que veria.
record-help-toggle = O que esta secção mostra

record-gender = Género
record-living = Vivo
record-visibility = Visibilidade
record-yes = sim
record-no = não
record-name-type = Tipo de nome
record-name-used = Em uso
record-name-evidence = Prova
record-transliteration = Transliteração latina
record-born = Nascido/a
record-died = Falecido/a
record-parents = Pais
record-siblings = Irmãos
record-children = Filhos
record-unknown-person = [Desconhecido]
record-restricted-person = Reservada
record-restricted-title = Esta ficha não lhe é visível
record-absent-person-title = Nomeado nesta árvore mas sem ficha própria
record-confidence = Certeza
record-source = Fonte
record-download = Transferir

## Acesso

access-restricted-title = Não visível para si
access-restricted-signed-in = A visibilidade desta ficha está acima do que a sua conta pode ler. Um administrador pode mudar ou a visibilidade da ficha ou o seu papel.
access-restricted-anonymous = Esta ficha não é pública. Entre para ver se a sua conta a pode ler.
access-role-title = Não para o seu papel
access-role-admin = Esta é uma página de administrador. A sua conta pode criar e editar fichas, mas não gerir contas, apagar fichas nem exportar o arquivo.
access-role-write = A sua conta pode ler esta árvore mas não alterá-la. Um administrador pode subir o seu papel a colaborador.
access-scope-title = Fora do seu ramo
access-scope-named = A sua conta está limitada a um ramo da árvore, e esta ficha diz respeito a alguém de fora. Cada pessoa nomeada numa ficha tem de estar dentro do seu ramo: caso contrário, uma família com um parceiro de fora seria uma maneira de reescrever a filiação dessa pessoa.
access-scope-unnamed = A sua conta está limitada a um ramo da árvore, e esta ficha não nomeia ninguém com quem a confrontar. As fontes e os lugares são editados por contas com acesso à árvore inteira.

## Erros

error-not-found-title = Não encontrado
error-not-found-detail = Essa página não existe aqui.
error-no-such-person-title = Não há tal pessoa
error-no-such-person-detail = Aqui não há nenhuma pessoa com esse identificador.
error-no-such-entity-title = Não há tal elemento
error-no-such-entity-detail = Aqui não há nenhuma ficha com esse identificador.
error-deleted-while-editing = Aqui não há nenhuma ficha com esse identificador. Pode ter sido apagada enquanto a editava.
error-no-such-file-title = Não há tal ficheiro
error-no-such-file-detail = Aqui não há nenhum documento com esse identificador, ou o documento está registado sem ficheiro — um documento citado nomeia algo guardado noutro sítio.
error-not-an-image-title = Não é uma imagem
error-not-an-image-detail = Para este documento não há miniatura, porque não é uma imagem que esta versão saiba descodificar.
error-back = Voltar

## Início de sessão

login-title = Entrar
login-lede = As contas são criadas por um administrador.
login-username = Nome de utilizador
login-password = Palavra-passe
login-submit = Entrar
login-wrong = Esse nome de utilizador e essa palavra-passe não correspondem.
login-token-wrong = Esse testemunho não está correcto.
login-throttled = Demasiadas tentativas falhadas. Espere alguns minutos e tente outra vez.
login-no-accounts-title = Esta instalação ainda não tem contas.
login-no-accounts-detail = Não há aqui página de configuração de propósito: o intervalo entre a entrada em serviço e o primeiro acesso é precisamente o momento em que uma instalação está desprotegida, por isso o primeiro administrador cria-se na linha de comandos.
login-no-accounts-note = Imprime uma palavra-passe gerada no stderr uma vez e nunca mais. Até lá a única entrada é o testemunho de emergência abaixo.
login-emergency-summary = Acesso de emergência
login-emergency-detail = O testemunho partilhado continua a abrir uma sessão de administrador e existe para uma coisa só: voltar a entrar quando o ficheiro .acl se perdeu ou todos os administradores ficaram de fora. Não é uma conta: não tem preferências próprias, e o diário de alterações anota-o como emergency-token em vez de como pessoa. O seu uso é registado como aviso.
login-emergency-label = Testemunho de emergência
login-emergency-submit = Usar o testemunho de emergência
login-sign-in-prompt = Entre para chegar ao painel de administração.

## Administração

admin-title = Administração
admin-lede = A editar { $path } — { $total } elementos, { $files ->
        [one] um ficheiro anexado
       *[other] { $files } ficheiros anexados
    }, { $size } em disco. Cada alteração é escrita de uma vez; uma alteração recusada deixa o ficheiro intacto.
admin-entities = Elementos
admin-create = Criar
admin-new-kind = Novo: { $kind }
admin-operations = Operações
admin-validate = Verificar
admin-deduplicate = Juntar duplicados
admin-export = Exportar o arquivo
admin-accounts = Contas
admin-roles-note = Verificar, juntar duplicados, exportar, apagar e gerir contas é só do administrador. Um colaborador chega a todas as outras páginas daqui.
admin-dedup-confirm = Juntar duplicados funde fichas e reescreve o arquivo. Continuar?
admin-recent-changes = Alterações recentes
admin-recent-note = As últimas { $shown } de { $total ->
        [one] uma alteração registada
       *[other] { $total } alterações registadas
    }, de { $path }.
admin-sessions-open = { $n ->
        [one] Uma sessão aberta neste momento.
       *[other] { $n } sessões abertas neste momento.
    }
admin-no-changes-yet = Com esta aplicação ainda não foi mudado nada. Cada gravação daqui em diante fica anotada em { $path }.
admin-last-validation = Última verificação
admin-bundle-heavy = Este arquivo pesa { $size }. É carregado por inteiro ao arrancar e mantido em memória, por isso acima de cerca de { $warn } o sítio começa a custar memória a sério e os reinícios ficam lentos. Isso serve a um arquivo de família, não a uma mediateca: se os anexos crescerem sem limite, guarde-os num depósito de ficheiros e faça o arquivo apontar para eles.

admin-fields = Campos
admin-raw-json = JSON em bruto
admin-raw-json-help = O elemento inteiro, para que nada fique por editar: listas como os parceiros e os filhos de uma família, ou a história de fronteiras de um lugar, vivem precisamente aqui. Este é o documento de partida; os campos acima são depois escritos sobre os caminhos que lhes pertencem, por isso edite um valor num sítio ou no outro, não nos dois. Tem de ler-se como JSON, senão não se guarda nada.
admin-save = Guardar
admin-cancel = Cancelar
place-editor-title = Editar um lugar
place-add-detail = Completar este lugar
place-names = Nomes
place-name-primary = Principal
place-name-lang = Língua
place-name-value = Nome
place-names-hint = One row per recorded name. A place administered by three empires carries three names; the primary is the one shown everywhere else.
place-where = Localização
place-type = Tipo
place-region = Região
place-country-current = País hoje
place-country-hint = ISO 3166-1 alpha-2, e.g. PL, FR, DE.
place-country-history = História das fronteiras
place-history-country = Estado
place-history-from = De
place-history-until = Até
place-country-history-hint = Which state held this place over which period. Genealogically significant: a record written in Russian in 1880 and one written in Polish in 1930 can name the same village.
place-coordinates = Coordenadas
place-lat = Latitude
place-lon = Longitude
place-precision = Precisão
place-identifiers = Identificadores
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } é usado por { $n ->
        [one] outro registo
       *[other] outros { $n } registos
    }.
place-error-no-name = A place needs at least one name.
place-error-coords-pair = Latitude and longitude go together: give both, or neither.
place-error-coords-number = Latitude and longitude must be numbers.
place-error-coords-range = Latitude runs -90 to 90 and longitude -180 to 180.
place-type-continent = continent
place-type-country = country
place-type-region = region
place-type-department = department
place-type-city = city
place-type-village = village
place-type-district = district
place-type-street = street
place-type-building = building
place-type-farm = farm
place-type-island = island
place-type-historical = historical
place-type-unknown = unknown
place-precision-exact = exact
place-precision-building = building
place-precision-street = street
place-precision-city_center = city centre
place-precision-region_center = region centre
place-precision-country_center = country centre
place-precision-approximate = approximate

place-coordinates-hint = Escrever à mão é o caminho habitual. Muitos lugares registados sob uma administração anterior não aparecem numa pesquisa moderna.
place-geocode-search = Procurar este nome
place-geocode-hint = Envia o nome, a região e o país ao serviço de geocodificação, um lugar de cada vez. Nada é guardado até que guarde.
place-geocode-off = A procura de nomes está desligada. Precisa de um endereço de contacto pelo qual o serviço identifique esta instalação; inicie o servidor com --geocoder-contact para a ligar.
place-geocode-query = Procurou-se: { $q }
place-geocode-error = Não foi possível contactar o serviço de procura. Os campos de coordenadas acima continuam a funcionar.
place-geocode-none = Nada encontrado. Para uma aldeia registada sob administração russa, prussiana ou austríaca este é o resultado comum; introduza a posição à mão.
place-geocode-not-a-place = não é uma povoação
place-geocode-use = Usar este
place-geocode-attribution = Resultados do OpenStreetMap via Nominatim, sob a Open Database License.
admin-delete = Apagar
admin-not-set = — por definir —
admin-edit = Editar
admin-page-of = Página { $page } de { $pages }
admin-previous = Anterior
admin-next = Seguinte
admin-saved = Guardado como versão { $version } — { $summary }
admin-not-saved = Não guardado
admin-created = Criado
admin-not-created = Não criado
admin-deleted = Apagado
admin-not-deleted = Não apagado — nada foi alterado
admin-what-changed = o que mudou
admin-field = Campo
admin-from = De
admin-to = Para
admin-version = versão { $version }

## Contas

accounts-title = Contas
accounts-lede = Guardadas em { $path }, com permissões 600, ao lado do arquivo e nunca dentro. Um arquivo copia-se, envia-se e publica-se; os resumos das palavras-passe a viajar lá dentro fariam de cada cópia da árvore uma cópia das credenciais.
accounts-existing = Existentes
accounts-username = Nome de utilizador
accounts-role = Papel
accounts-status = Estado
accounts-branch = Ramo
accounts-last-seen = Último acesso
accounts-change = Alterar
accounts-you = (você)
accounts-active = activa
accounts-disabled = desactivada
accounts-never = nunca
accounts-whole-tree = árvore inteira
accounts-roots = { $n ->
        [one] uma raiz
       *[other] { $n } raízes
    }
accounts-add = Adicionar uma conta
accounts-no-registration = De propósito não há inscrição por conta própria nem convites. Para um arquivo de família basta um administrador que conheça toda a gente, e isso retira por inteiro uma superfície de abuso em vez de ter de a defender.
accounts-password-hint = Deixe em branco e é gerada uma, mostrada uma só vez. Pelo menos { $min } caracteres se a definir você.
accounts-new-password-placeholder = nova palavra-passe (em branco = manter)
accounts-email = Correio electrónico
accounts-optional = (facultativo)
accounts-create = Criar a conta
accounts-role-viewer = leitura — lê as fichas públicas e as da família
accounts-role-contributor = colaborador — além disso cria, edita e envia ficheiros
accounts-role-admin = administrador — além disso gere contas, apaga e exporta
accounts-branch-hint = Limita o que esta conta pode editar a essas pessoas, aos seus descendentes e aos seus cônjuges.
accounts-branch-reading = Não limita o que pode ler: isso é governado pela visibilidade de cada ficha, e as duas coisas mantêm-se separadas de propósito.
accounts-branch-placeholder = um identificador de pessoa por linha
accounts-ids-in-bundle = Identificadores de pessoas nesta árvore
accounts-emergency-warning = Entrou com o testemunho de emergência. Concede direitos de administrador nesta sessão mas não é uma conta: não tem preferências próprias, e o diário anotará as suas alterações como emergency-token em vez de como pessoa. Crie abaixo uma conta a sério e entre com ela.
accounts-created-with-password = Criada { $username }. A palavra-passe é { $password } — é mostrada uma só vez e guardada apenas como resumo Argon2id, por isso passe-a agora.
accounts-created = Criada { $username }.
accounts-updated = Actualizada { $username }. Qualquer sessão que tivesse aberta foi terminada.
accounts-username-taken = Esse nome de utilizador já está ocupado.
accounts-pick-role = Escolha um papel.
accounts-no-such = Não existe essa conta.
accounts-last-admin = É o único administrador activo. Promova antes outra pessoa: uma instalação sem administrador só se recupera editando o ficheiro .acl ou usando o testemunho de emergência.
accounts-not-saved = Não guardado: { $error }

## Conflitos

conflict-title = Outra pessoa alterou isto primeiro
conflict-lede = { $who } guardou uma alteração a este elemento ({ $kind }) às { $when }, depois de o ter aberto. A sua edição não foi guardada e nada foi substituído.
conflict-no-merge = Aqui nada se junta automaticamente. Fundir as edições de duas pessoas produz uma ficha que nenhuma delas escolheu, e em genealogia dois redactores em desacordo sobre uma data costumam estar a ler fontes diferentes — e isso é uma pergunta para uma pessoa, não para um programa. Compare as duas abaixo e decida.
conflict-versions = Partiu da versão { $expected }; a ficha está agora na versão { $current }.
conflict-both-changed = Alteraram isto os dois
conflict-both-changed-detail = Estes campos foram editados por ambos. O que quer que guarde substituirá o que { $who } lá pôs:
conflict-different-fields = Alteraram campos diferentes, por isso nada do trabalho de { $who } está em causa — mas voltar a aplicar escreve na mesma o seu elemento inteiro por cima do da outra pessoa. Verifique as duas colunas antes de guardar.
conflict-field-by-field = Campo a campo
conflict-theirs = Para o que { $who } o mudou
conflict-yours = Para o que você o mudou
conflict-unchanged-by-you = não alterado por si
conflict-unchanged-by-them = não alterado por eles
conflict-nothing-differs = Nenhuma das versões difere daquela de que partiu em nenhum campo que esta página mostre. O número de versão avançou, portanto alguém guardou a ficha sem mudar nada do que ela contém.
conflict-what-now = E agora
conflict-reapply = Voltar a aplicar a sua versão por cima da deles
conflict-reapply-hint = Esta é a sua edição, transportada para a versão { $version }. Corrija-a aqui para manter o que quiser do trabalho de { $who } e depois guarde. A versão deles está abaixo, para copiar.
conflict-save-over = Guardar esta por cima da deles
conflict-discard = Descartar a minha e recomeçar
conflict-their-version = A versão de { $who }, tal como está agora
conflict-history-of = Histórico deste elemento ({ $kind })

## Importação

convert-title = Importar um ficheiro familiar
convert-submit = Importar
convert-result-title = Relatório de importação
convert-download = Transferir o arquivo

## Datas

date-unknown = Data desconhecida
date-not-recorded = Não registada
date-circa = cerca de { $date }
date-between = entre { $from } e { $to }
date-before = antes de { $date }
date-after = depois de { $date }
date-preserved = registada como «{ $text }»
date-day-month-year = { $day } de { $month ->
        [1] janeiro
        [2] fevereiro
        [3] março
        [4] abril
        [5] maio
        [6] junho
        [7] julho
        [8] agosto
        [9] setembro
        [10] outubro
        [11] novembro
        [12] dezembro
        *[other] { $month }
    } de { $year }
date-month-year = { $month ->
        [1] janeiro
        [2] fevereiro
        [3] março
        [4] abril
        [5] maio
        [6] junho
        [7] julho
        [8] agosto
        [9] setembro
        [10] outubro
        [11] novembro
        [12] dezembro
        *[other] { $month }
    } de { $year }
date-decade = os anos { $decade }
date-century = o século { $century }
date-quarter-century = o { $quarter ->
        [1] primeiro
        [2] segundo
        [3] terceiro
       *[other] quarto
    } quartel do século { $century }

## Mais páginas de erro

error-back-to-start = Voltar ao início
error-payload-missing-title = Não há tal ficheiro
error-payload-missing-detail = O conteúdo desse documento não está na cache.
error-payload-unopenable-detail = O conteúdo desse documento não se conseguiu abrir.
error-no-such-document-detail = Aqui não há nenhum documento com esse identificador.
error-bad-preference-title = Não é uma das opções
error-bad-preference-detail = Não é uma língua nem um aspecto que este sítio ofereça. Nada foi alterado.
error-unknown-kind-title = Espécie desconhecida
error-unknown-kind-detail = «{ $kind }» não é uma espécie de ficha. Este arquivo contém: { $kinds }.
error-io-title = Não foi possível guardar
error-io-detail = { $error }. No disco nada foi alterado.
error-upload-too-large = Esse ficheiro passa o limite de { $mb } MB. Nada foi guardado e o arquivo fica igual.
error-upload-refused = O documento foi recusado: { $reason }. O arquivo fica igual.
error-back-to-person = Voltar à ficha
error-no-such-person-to-attach = Aqui não há nenhuma pessoa com esse identificador, portanto não há a que anexar um documento.
error-upload-title = Esse envio não foi guardado
error-download-expired-title = Essa transferência expirou
error-download-expired-detail = Uma importação guarda-se quinze minutos e depois é descartada. Importe o ficheiro outra vez.
error-upload-none = Não foi enviado nenhum ficheiro. Escolha primeiro um.
error-upload-unsupported = Esse tipo de ficheiro o arquivo não guarda. Aceitam-se imagens, PDF, texto simples, áudio e vídeo; o tipo lê-se dos próprios bytes do ficheiro, por isso mudar o nome a um executável não serve. O SVG é recusado sem mais, porque um SVG pode levar um script.
error-export-unreadable-title = Não foi possível ler o arquivo exportado
error-export-unreadable-detail = { $error }

## Página da árvore, continuação

tree-title-suffix = árvore
tree-back-to-focused = Voltar à vista à volta de uma pessoa
tree-show-all = Mostrar as { $n }
tree-width-notice = Esta vista tem { $width } pixéis de largura: num ecrã de 1500 pixéis são { $screens ->
        [one] um ecrã
       *[other] { $screens } ecrãs
    } de deslocamento na horizontal.
tree-confidence-label = Certeza:
tree-band-certain = certo
tree-band-high = alta
tree-band-medium = média
tree-band-low = hipotético
tree-counts = { $drawn } de { $total } pessoas · { $generations ->
        [one] uma geração
       *[other] { $generations } gerações
    }
tree-unplaced-count = { $n } sem lugar
tree-contradicts-title = Esta árvore contradiz-se.
tree-contradicts-detail = Nenhuma disposição de linhas pode satisfazer isso, por isso o parentesco abaixo ficou fora da numeração das gerações e alguma linha pode estar desenhada no sítio errado. Corrija aquela das duas fichas que está errada.
tree-contradicts-pair = Registados ao mesmo tempo como casal e como progenitor e filho:
tree-contradicts-more = { $n ->
        [one] Outra contradição não está listada.
       *[other] Outras { $n } contradições não estão listadas.
    }
tree-no-people = Nesta árvore ainda não há ninguém.
tree-no-people-cta = Importe um ficheiro familiar, ou acrescente a primeira pessoa.
tree-nobody-selected = Para essa selecção não há ninguém para desenhar.
tree-nobody-selected-cta = Comece pela vista por omissão.
tree-edge-union = Uma união registada
tree-edge-parentage = Uma filiação registada

## Página inicial

home-empty = Ainda não há nada registado. Importe um ficheiro familiar para trazer uma árvore já existente, ou acrescente à mão a primeira pessoa.
home-count = { $total ->
        [one] Uma ficha
       *[other] { $total } fichas
    }, num único ficheiro que é da família.
home-browse = Percorrer a árvore
home-convert = Importar um ficheiro familiar
home-unnamed-family = Esta árvore de família
home-in-this-tree = O que a família registou até agora
home-showcase-title = Onde esta árvore já diz mais do que nomes e datas
home-showcase-example = Ver um exemplo →
home-nothing-title = Ainda não há nada para mostrar.
home-nothing-detail = Importe um ficheiro familiar para trazer uma árvore já existente, ou comece do zero e acrescente você mesmo a primeira pessoa.

## Cartões de mostra

showcase-links-title = { $n ->
        [one] Uma relação fora da família
       *[other] { $n } relações fora da família
    }
showcase-links-detail = Padrinhos, patrões, testemunhas e mestres, cada um com as suas próprias datas, a sua fonte e o seu grau de certeza.
showcase-occupations-title = { $n ->
        [one] Uma ocupação com um princípio e um fim
       *[other] { $n } ocupações com um princípio e um fim
    }
showcase-occupations-detail = «Professora primária, 1948-1978» conserva a sua duração e é desenhada como uma barra ao longo dos anos, não como uma única linha datada.
showcase-uncertain-title = { $n ->
        [one] Uma data deixada tão imprecisa como foi dada
       *[other] { $n } datas deixadas tão imprecisas como foram dadas
    }
showcase-uncertain-detail = Cerca de, antes, depois e entre continuam a ser quatro afirmações diferentes. Uma data que a fonte não soube fixar nunca é mostrada como se a tivesse fixado.
showcase-preserved-title = { $n ->
        [one] Uma data guardada nas palavras em que foi escrita
       *[other] { $n } datas guardadas nas palavras em que foram escritas
    }
showcase-preserved-detail = Uma formulação que ninguém soube ler como data fica exactamente como está escrita, em vez de ser descartada em silêncio.
showcase-sources-title = { $n ->
        [one] Uma fonte com a sua fiabilidade registada
       *[other] { $n } fontes com a sua fiabilidade registada
    }
showcase-sources-detail = { $primary ->
        [one] Uma fonte primária.
       *[other] { $primary } primárias.
    } Cada facto mostra em que prova se apoia e que força tem essa prova.
showcase-places-title = { $n ->
        [one] Um lugar cujas fronteiras se moveram
       *[other] { $n } lugares cujas fronteiras se moveram
    }
showcase-places-detail = Uma cidade pode pertencer a estados diferentes em épocas diferentes, e a ficha diz qual valia quando.

## Detalhes da ficha

record-also-recorded-as = registado também como
record-borders-moved = Fronteiras movidas:
record-display-name = nome mostrado
record-read-as = lido como
record-note = Nota
record-living-yes = vivo
record-deceased = falecido/a
record-centre-tree-here = Centrar a árvore aqui
record-centre-tree-title = Mover a árvore para a centrar nesta pessoa
record-open-full-page = Abrir a página inteira ↗
record-open-full-title = Abrir a página independente que se pode partilhar
record-edit = Editar
panel-empty = Escolha uma ficha para ver aqui o documento completo dessa pessoa.
person-see-in-tree = Ver esta pessoa na árvore
person-visibility-inline = visibilidade:
person-age-at-death = morreu aos { $n }
person-age-now = { $n } anos
person-born-in = nascido em { $place }
person-died-in = morreu em { $place }
person-children-count = { $n ->
        [one] um filho
       *[other] { $n } filhos
    }
person-generations-below = { $n ->
        [one] uma geração abaixo
       *[other] { $n } gerações abaixo
    }
person-portrait-of = Fotografia de { $name }
person-no-portrait = Sem fotografia registada

## Resultados das operações

result-diagnostics = Avisos
result-diagnostics-note = Todos os avisos devolvidos pela biblioteca, incluindo as advertências que não travaram a operação. Nenhum é filtrado.
result-no-diagnostics = A biblioteca não devolveu avisos.
result-continue = Continuar
result-dashboard = Painel
person-sections-label = Secções desta página

## Secções da ficha, detalhes

record-notes-title = A reter sobre esta ficha:
record-name = Nome
record-type = Tipo
record-cause = Causa:
record-as = como
record-partner-not-recorded = Parceiro não registado
record-union-from = Desde
record-union-at = em
record-union-until = até
record-occupation-from = desde
record-occupation-until = até
record-source-reliability = Fiabilidade
record-source-supports = Sustenta
record-photographs = Fotografias
record-documents = Documentos
record-file = Ficheiro
record-status = Estado
record-size = Tamanho
record-absent-document = Nomeado por esta pessoa mas não guardado aqui.
record-no-file = sem ficheiro
record-attach-document = Anexar um documento
record-doc-photo = foto
record-doc-certificate = assento
record-doc-letter = carta
record-doc-record = registo
record-doc-newspaper = jornal
record-doc-other = outro
record-upload = Enviar
record-upload-help = Até { $mb } MB por ficheiro. Os anexos ficam ao lado da árvore e são reescritos no arquivo ao exportar, por isso uma fotografia viaja com a família a que pertence. A espécie de ficheiro lê-se do seu próprio conteúdo e não do nome: aceitam-se imagens, PDF, texto simples, áudio e vídeo. O SVG é recusado, porque um SVG pode levar um script.
record-upload-help-short = Até { $mb } MB. O SVG é recusado.
record-verbatim-note = Guardado tal como a ficha o dava, porque nenhum conversor o soube interpretar.
record-file-to-attach = Ficheiro a anexar
record-document-type = Tipo de documento
record-caption = Legenda
record-caption-placeholder = Legenda (facultativa)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Espécies de elemento

kind-person = pessoa
kind-family = família
kind-event = facto
kind-link = vínculo
kind-occupation = ocupação
kind-source = fonte
kind-place = lugar
kind-document = documento

kind-person-plural = { $n ->
        [one] pessoa
       *[other] pessoas
    }
kind-family-plural = { $n ->
        [one] família
       *[other] famílias
    }
kind-event-plural = { $n ->
        [one] facto
       *[other] factos
    }
kind-link-plural = { $n ->
        [one] vínculo
       *[other] vínculos
    }
kind-occupation-plural = { $n ->
        [one] ocupação
       *[other] ocupações
    }
kind-source-plural = { $n ->
        [one] fonte
       *[other] fontes
    }
kind-place-plural = { $n ->
        [one] lugar
       *[other] lugares
    }
kind-document-plural = { $n ->
        [one] documento
       *[other] documentos
    }

## Listagens

list-matching = { $total ->
        [one] Uma correspondência
       *[other] { $total } correspondências
    }, { $per_page } por página.
list-filter-placeholder = Filtrar por nome ou identificador
list-filter = Filtrar
list-clear = Limpar
list-summary = Descrição
list-id = Identificador
list-actions = Acções
list-nothing = Aqui não há nada.
list-nothing-matching = Aqui não há nada que corresponda a «{ $q }».
list-delete-confirm = Apagar este elemento ({ $kind })? Escolha o que acontece aos elementos que o citam:
list-policy-reject = Recusar
list-policy-reject-detail = — recusar se alguma coisa ainda o citar. Nada se perde.
list-policy-cascade = Em cascata
list-policy-cascade-detail = — apagá-lo e retirar mesmo todas as citações a ele.
list-policy-orphan = Deixar órfãos
list-policy-orphan-detail = — apagá-lo mas manter as fichas que o citam, com a citação esvaziada.

## Grau de detalhe

completeness-dates-title = As datas segundo a forma que realmente têm
completeness-no-dates = Ainda não há datas registadas.
completeness-dates-note = Uma data que alguém soube fixar ao dia e outra que alguém só soube situar numa década são duas afirmações diferentes, e ambas ficam como foram dadas. O texto que não se conseguiu ler como data guarda-se palavra por palavra em vez de ser descartado.
completeness-shape-exact = exacta
completeness-shape-exact-note = um dia de calendário inteiro
completeness-shape-approximate = aproximada
completeness-shape-approximate-note = cerca de, ou só um ano ou uma década
completeness-shape-ranged = delimitada
completeness-shape-ranged-note = antes, depois ou entre
completeness-shape-preserved = à letra
completeness-shape-preserved-note = texto não interpretável, guardado tal e qual
completeness-shape-unknown = desconhecida
completeness-shape-unknown-note = registada como não sabida

## Página de importação

convert-page-title = Importar um ficheiro familiar
convert-lede = Traga uma árvore já existente a partir de um ficheiro GEDCOM, a exportação que a maior parte dos programas de genealogia produz. Aqui não se guarda nada, e a árvore que este sítio já mostra fica exactamente como estava.
convert-file-label = Ficheiro familiar (.ged)
convert-file-hint = Até { $mb } MB. Uma árvore de 767 pessoas pesa cerca de 320 KB.
convert-confidence-label = Que certeza têm estes factos, para começar
convert-confidence-hint = O ficheiro a importar não diz que segurança alguém tinha, por isso cada facto precisa de um ponto de partida. Ponha-o baixo para uma árvore reunida à pressa, mais alto para uma trabalhada sobre documentos. A leitura honesta deste número é «importado, e desde então ninguém verificou»: poderá subir ou baixar cada facto depois, um a um.
convert-lang-label = Língua dos nomes de lugar
convert-lang-hint = Uma etiqueta como en, fr ou pt.

## Relatório de importação

convert-failed = A importação não foi por diante
convert-try-another = Tentar outro ficheiro
convert-converted = Importado { $filename }
convert-result-lede = { $total ->
        [one] Uma ficha
       *[other] { $total } fichas
    }, { $size } KB. Entrou tudo com uma certeza de { $confidence }, com os nomes de lugar lidos como { $lang }. A árvore que este sítio mostra não foi tocada.
convert-produced = O que passou
convert-skipped-title = { $n ->
        [one] Uma entrada que não se conseguiu ler
       *[other] { $n } entradas que não se conseguiram ler
    }
convert-skipped-note = Estas entradas não continham nada que se pudesse trazer.
convert-other-diagnostics = { $n ->
        [one] Outra coisa que convém saber
       *[other] Outras { $n } coisas que convém saber
    }
convert-clean = Nada ficou para trás — todas as entradas do ficheiro passaram.
convert-download-title = Transferência
convert-download-named = Transferir { $name }
convert-download-note = Guardado aqui quinze minutos e depois descartado, por isso transfira-o agora.
convert-another = Importar outro ficheiro
admin-history-on = a
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Um erro
       *[other] { $errors } erros
    }, { $warnings ->
        [one] uma advertência
       *[other] { $warnings } advertências
    }, { $infos ->
        [one] uma nota
       *[other] { $infos } notas
    }.
admin-warnings-never-block = As advertências nunca travam nada: são informação, não uma barreira.
admin-validator-clean = A verificação não comunicou nada.
record-occupations-help-undated = Uma ocupação regista-se com princípio e fim, de modo que várias se comparem sobre uma mesma linha de tempo. Este arquivo tem os nomes dos ofícios mas não as datas — coisa normal depois de uma importação, porque a maior parte dos ficheiros familiares não tem onde as guardar —, por isso ainda não há escala que desenhar.
record-occupations-help-axis = Uma ocupação é um estado com duração, não um facto numa só data. Todos os troços partilham um eixo, { $from }–{ $to }.
admin-value-not-set = por definir
admin-validation-report = Relatório de verificação
admin-dedup-complete = Junção de duplicados concluída
admin-dedup-refused = Junção de duplicados recusada
record-birth-order = ordem de nascimento
record-start-not-recorded = princípio não registado
record-end-not-recorded = fim não registado
record-document-no-file = O documento está registado aqui, mas o ficheiro em si não está
panel-selected-person = Pessoa escolhida

## Faixas de geração

tree-band-generation = Geração { $g }
tree-band-people = { $n ->
        [one] uma pessoa
       *[other] { $n } pessoas
    }
tree-band-unplaced = Sem lugar
tree-band-unplaced-note = { $n ->
        [one] uma pessoa sem família — mostrada em vez de omitida
       *[other] { $n } pessoas sem família — mostradas em vez de omitidas
    }

## Vocabulário controlado

gender-M = Masculino
gender-F = Feminino
gender-NB = Não binário
gender-unrecorded = Não registado

name-part-given_name = nome próprio
name-part-family_name = apelido
name-part-patronymic = patronímico
name-part-matronymic = matronímico
name-part-middle_name = nome do meio
name-part-nickname = alcunha
name-part-prefix = prefixo
name-part-suffix = sufixo
name-part-particle = partícula
name-part-part = elemento

name-type-primary = principal
name-type-other = outro
name-type-alias = de uso
name-type-birth = de nascimento
name-type-married = de casada
name-type-religious = religioso
name-type-transliteration = transliteração
name-type-nickname = alcunha

## Anotações sobre a ficha

note-links = { $n ->
        [one] uma relação fora da família, com datas e fontes próprias
       *[other] { $n } relações fora da família, com datas e fontes próprias
    }
note-occupations = { $n ->
        [one] um ofício registado com princípio e fim
       *[other] { $n } ofícios registados com princípio e fim
    }
note-birth-imprecise = uma data de nascimento que a fonte não soube fixar, mostrada tal como está registada
note-death-imprecise = uma data de óbito que a fonte não soube fixar, mostrada tal como está registada
note-names = { $n ->
        [one] um nome registado
       *[other] { $n } nomes registados
    }
note-transliteration = um nome na sua própria escrita ao lado da transliteração latina
note-witnessed = { $n ->
        [one] um facto de que foi testemunha e não protagonista
       *[other] { $n } factos de que foi testemunha e não protagonista
    }

visibility-public = pública
visibility-members = familiares
visibility-contributors = colaboradores
visibility-private = reservada

## Descrições de linha nas listagens de administração

family-label-couple = { $children ->
        [0] { $a } e { $b }
        [one] { $a } e { $b } — um filho
       *[other] { $a } e { $b } — { $children } filhos
    }
family-label-half = { $children ->
        [0] { $a } e { $unknown }
        [one] { $a } e { $unknown } — um filho
       *[other] { $a } e { $unknown } — { $children } filhos
    }
family-label-children = { $others ->
        [0] { $first } — pais não registados
        [one] { $first } e um irmão — pais não registados
       *[other] { $first } e { $others } irmãos — pais não registados
    }
family-label-empty = Família sem ninguém registado

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } e { $b }
event-more-people = { $a } e { $b } e mais { $others ->
        [one] um
       *[other] { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } sem título
list-unnamed = { $kind } sem nome

## Vocabulários da especificação nas listagens

event-category-birth = Nascimento
event-category-death = Óbito
event-category-marriage = Casamento
event-category-divorce = Divórcio
event-category-baptism = Baptismo
event-category-burial = Enterro
event-category-immigration = Imigração
event-category-emigration = Emigração
event-category-census = Recenseamento
event-category-residence = Residência
event-category-military = Serviço militar
event-category-education = Estudos
event-category-other = Facto

reliability-primary = fonte primária
reliability-secondary = fonte secundária
reliability-tertiary = fonte terciária
reliability-recollection = testemunho oral
reliability-derivative = obra derivada
reliability-authored = obra de autor
reliability-oral = tradição oral
reliability-unknown = fiabilidade desconhecida

document-type-photo = fotografia
document-type-certificate = assento
document-type-letter = carta
document-type-record = registo de arquivo
document-type-newspaper = recorte de jornal
document-type-other = documento

## Onde esta ficha poderia dizer mais

completeness-title = Onde esta árvore poderia dizer mais
completeness-intro = O que está registado e o que continua em branco.
completeness-import-title = O que a importação trouxe
completeness-import-intro = Contado sobre o ficheiro que acabou de enviar. Uma linha em branco é algo que o ficheiro de origem não registava, não algo que a importação tenha perdido.

completeness-headline-full = Cada espécie de pormenor abaixo está registada nalgum sítio desta árvore.
completeness-headline-empty = { $total ->
        [one] A única espécie de pormenor abaixo ainda não está registada em lado nenhum.
       *[other] Nenhuma das { $total } espécies de pormenor abaixo está ainda registada.
    } Cada uma é um sítio onde a ficha poderia dizer mais.
completeness-headline-partial = { $carried ->
        [one] Uma espécie de pormenor abaixo está registada
       *[other] { $carried } espécies de pormenor abaixo estão registadas
    }; { $empty ->
        [one] uma continua em branco
       *[other] { $empty } continuam em branco
    }.

completeness-metric-confidence = Que certeza tem cada facto
completeness-metric-confidence-none = Nenhum dos { $slots } factos aqui diz que certeza tem. Uma data lida num assento e outra adivinhada parecem-se, até deixarem de se parecer.
completeness-metric-confidence-uniform = { $with } de { $slots } factos levam uma pontuação, e todas são o mesmo número ({ $modal }). É o que uma importação em bloco deixa atrás de si: um valor de recurso a que ninguém voltou. Nenhum foi ainda julgado um a um.
completeness-metric-confidence-some = { $with } de { $slots } factos levam uma pontuação. { $modal_count } partilham um mesmo valor ({ $modal }); { $assessed } afastam-se dele e foram por isso vistos um de cada vez.
completeness-metric-confidence-many = { $with } de { $slots } factos levam uma pontuação, e { $assessed } deles afastam-se do valor mais frequente ({ $modal }), ao longo de { $distinct } níveis distintos. Esta árvore regista uma incerteza real e variada.

completeness-metric-parentage = Que certeza tem cada vínculo pai-filho
completeness-metric-parentage-none = Nenhuma filiação aqui diz que certeza tem. As adopções, as linhas contestadas e as reconstituições a partir de uma só menção são justamente os sítios onde uma família precisa de registar a dúvida — e a árvore desenha um vínculo menos seguro com uma linha mais pálida.
completeness-metric-parentage-some = { $n ->
        [one] Uma filiação leva a sua própria pontuação
       *[other] { $n } filiações levam a sua própria pontuação
    }, por isso uma linha hipotética é visivelmente mais fraca do que uma documentada.

completeness-metric-links = Relações para além do sangue e do casamento
completeness-metric-links-none = Padrinhos, patrões, testemunhas, mestres, tutores. Ainda não há nenhuma registada. Cada uma pode levar as suas próprias datas, a sua fonte e o seu grau de certeza.
completeness-metric-links-some = { $n ->
        [one] Uma registada, com as suas próprias datas, fonte e o seu grau de certeza.
       *[other] { $n } registadas, cada uma com as suas próprias datas, fonte e o seu grau de certeza.
    }

completeness-metric-occupations = Ofícios registados com princípio e fim
completeness-metric-occupations-none = Não há ocupações registadas. Um ofício exercido durante trinta anos diz mais de uma vida do que uma única entrada datada.
completeness-metric-occupations-undated = { $total ->
        [one] Está registada uma ocupação, sem datas
       *[other] Estão registadas { $total } ocupações, sem datas
    }. Acrescente um princípio e um fim e poderão comparar-se lado a lado sobre uma mesma linha de tempo.
completeness-metric-occupations-some = { $span } de { $total } têm princípio ou fim, portanto podem comparar-se lado a lado sobre uma mesma linha de tempo.

completeness-metric-sources = Fontes com a fiabilidade avaliada
completeness-metric-sources-none = Não há fontes registadas. Dizer de onde veio um facto é o que permite a um parente verificá-lo mais tarde — ou discordar e dizer porquê.
completeness-metric-sources-some = { $graded } de { $total } dizem que força têm, por isso uma afirmação que se apoia num assento de nascimento não é visivelmente o mesmo que uma que se apoia numa recordação.

completeness-what-is-recorded = O que a ficha pode dizer
completeness-in-this-tree = Nesta árvore
completeness-not-yet = ainda não registado

## Papéis de um participante num facto

role-spouse = cônjuge
role-spouse_1 = primeiro cônjuge
role-spouse_2 = segundo cônjuge
role-subject = pessoa do registo
role-participant = participante
role-witness = testemunha
role-officiant = celebrante
role-informant = declarante
role-godparent = padrinho ou madrinha
