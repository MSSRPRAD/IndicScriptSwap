// use crate::read_mappings::read_mappings;

// use crate::read_mappings::ScriptData;
mod convert;
mod data;
mod functions;
mod read_mappings;

fn main() {
    let foo = &data::HASH_MAP;

    let mut destination = foo.get("hk").unwrap();
    let mut source = foo.get("iast").unwrap();

    let input = "asty uttarasyāṃ diśi devatātmā himālayo nāma nagādhirājaḥ .
    pūrvāparau toyanidhī vigāhya sthitaḥ pṛthivyā iva mānadaṇḍaḥ ..
    yaṃ sarvaśailāḥ parikalpya vatsaṃ merau sthite dogdhari dohadakṣe .
    bhāsvanti ratnāni mahauṣadhīś ca pṛthūpadiṣṭāṃ duduhur dharitrīm ..
    anantaratnaprabhavasya yasya himaṃ na saubhāgyavilopi jātam .
    eko hi doṣo guṇasaṃnipāte nimajjatīndoḥ kiraṇeṣv ivāṅkaḥ ..
    yaś cāpsarovibhramamaṇḍanānāṃ saṃpādayitrīṃ śikharair bibharti .
    balāhakacchedavibhaktarāgām akālasaṃdhyām iva dhātumattām ..
    āmekhalaṃ saṃcaratāṃ ghanānāṃ cchāyām adhaḥsānugatāṃ niṣevya .
    udvejitā vṛṣṭibhir āśrayante śṛṅgāṇi yasyātapavanti siddhāḥ ..
    padaṃ tuṣārasrutidhautaraktaṃ yasminn adṛṣṭvāpi hatadvipānām .
    vidanti mārgaṃ nakharandhramuktair muktāphalaiḥ kesariṇāṃ kirātāḥ ..
    nyastākṣarā dhāturasena yatra bhūrjatvacaḥ kuñjarabinduśoṇāḥ .
    vrajanti vidyādharasundarīṇām anaṅgalekhakriyayopayogam ..
    yaḥ pūrayan kīcakarandhrabhāgān darīmukhotthena samīraṇena .
    udgāsyatām icchati kiṃnarāṇāṃ tānapradāyitvam ivopagantum ..
    kapolakaṇḍūḥ karibhir vinetuṃ vighaṭṭitānāṃ saraladrumāṇām .
    yatra srutakṣīratayā prasūtaḥ sānūni gandhaḥ surabhīkaroti ..
    vanecarāṇāṃ vanitāsakhānāṃ darīgṛhotsaṅganiṣaktabhāsaḥ .
    bhavanti yatrauṣadhayo rajanyām atailapūrāḥ suratapradīpāḥ ..
    udvejayaty aṅgulipārṣṇibhāgān mārge śilībhūtahime 'pi yatra .
    na durvahaśroṇipayodharārtā bhindanti mandāṃ gatim aśvamukhyaḥ ..
    divākarād rakṣati yo guhāsu līnaṃ divā bhītam ivāndhakāram .
    kṣudre 'pi nūnaṃ śaraṇaṃ prapanne mamatvam uccaiḥśirasāṃ satīva ..
    lāṅgūlavikṣepavisarpiśobhair itas tataś candramarīcigauraiḥ .
    yasyārthayuktaṃ girirājaśabdaṃ kurvanti vālavyajanaiś camaryaḥ ..
    yatrāṃśukākṣepavilajjitānāṃ yadṛcchayā kiṃpuruṣāṅganānām .
    darīgṛhadvāravilambibimbās tiraskariṇyo jaladā bhavanti ..
    bhāgīrathīnirjharasīkarāṇāṃ voḍhā muhuḥ kampitadevadāruḥ .
    yad vāyur anviṣṭamṛgaiḥ kirātair āsevyate bhinnaśikhaṇḍibarhaḥ ..
    saptarṣihastāvacitāvaśeṣāṇy adho vivasvān parivartamānaḥ .
    padmāni yasyāgrasaroruhāṇi prabodhayaty ūrdhvamukhair mayūkhaiḥ ..
    yajñāṅgayonitvam avekṣya yasya sāraṃ dharitrīdharaṇakṣamaṃ ca .
    prajāpatiḥ kalpitayajñabhāgaṃ śailādhipatyaṃ svayam anvatiṣṭhat ..
    sa mānasīṃ merusakhaḥ pitṝṇāṃ kanyāṃ kulasya sthitaye sthitijñaḥ .
    menāṃ munīnām api mānanīyām ātmānurūpāṃ vidhinopayeme ..
    kālakrameṇātha tayoḥ pravṛtte svarūpayogye surataprasaṅge .
    manoramaṃ yauvanam udvahantyā garbho 'bhavad bhūdhararājapatnyāḥ ..
    asūta sā nāgavadhūpabhogyaṃ mainākam ambhonidhibaddhasakhyam .
    kruddhe 'pi pakṣacchidi vṛtraśatrāv avedanājñaṃ kuliśakṣatānām ..
    athāvamānena pituḥ prayuktā dakṣasya kanyā bhavapūrvapatnī .
    satī satī yogavisṛṣṭadehā tāṃ janmane śailavadhūṃ prapede ..
    sā bhūdharāṇām adhipena tasyāṃ samādhimatyām udapādi bhavyā .
    samyakprayogād aparikṣatāyāṃ nītāv ivotsāhaguṇena saṃpat ..
    prasannadik pāṃsuviviktavātaṃ śaṅkhasvanānantarapuṣpavṛṣṭi .
    śarīriṇāṃ sthāvarajaṅgamānāṃ sukhāya tajjanmadinaṃ babhūva ..
    tayā duhitrā sutarāṃ savitrī sphuratprabhāmaṇḍalayā cakāse .
    vidūrabhūmir navameghaśabdād udbhinnayā ratnaśalākayeva ..
    dine dine sā parivardhamānā labdhodayā cāndramasīva lekhā .
    pupoṣa lāvaṇyamayān viśeṣāñ jyotsnāntarāṇīva kalāntarāṇi ..
    tāṃ pārvatīty ābhijanena nāmnā bandhupriyāṃ bandhujano juhāva .
    u meti mātrā tapaso niṣiddhā paścād umākhyāṃ sumukhī jagāma ..
    mahībhṛtaḥ putravato 'pi dṛṣṭis tasminn apatye na jagāma tṛptim .
    anantapuṣpasya madhor hi cūte dvirephamālā saviśeṣasaṅgā ..
    prabhāmahatyā śikhayeva dīpas trimārgayeva tridivasya mārgaḥ .
    saṃskāravatyeva girā manīṣī tayā sa pūtaś ca vibhūṣitaś ca ..
    mandākinīsaikatavedikābhiḥ sā kandukaiḥ kṛtrimaputrakaiś ca .
    reme muhur madhyagatā sakhīnāṃ krīḍārasaṃ nirviśatīva bālye ..
    tāṃ haṃsamālāḥ śaradīva gaṅgāṃ mahauṣadhiṃ naktam ivātmabhāsaḥ .
    sthiropadeśām upadeśakāle prapedire prāktanajanmavidyāḥ ..
    asaṃbhṛtaṃ maṇḍanam aṅgayaṣṭer anāsavākhyaṃ karaṇaṃ madasya .
    kāmasya puṣpavyatiriktam astraṃ bālyāt paraṃ sātha vayaḥ prapede ..
    unmīlitaṃ tūlikayeva citraṃ sūryāṃśubhir bhinnam ivāravindam .
    babhūva tasyāś caturasraśobhi vapur vibhaktaṃ navayauvanena ..
    abhyunnatāṅguṣṭhanakhaprabhābhir nikṣepaṇād rāgam ivodgirantau .
    ājahratus taccaraṇau pṛthivyāṃ sthalāravindaśriyam avyavasthām ..
    sā rājahaṃsair iva saṃnatāṅgī gateṣu līlāñcitavikrameṣu .
    vyanīyata pratyupadeśalubdhair āditsubhir nūpurasiñjitāni ..
    vṛttānupūrve ca na cātidīrghe jaṅghe śubhe sṛṣṭavatas tadīye .
    śeṣāṅganirmāṇavidhau vidhātur lāvaṇya utpādya ivāsa yatnaḥ ..
    nāgendrahastās tvaci karkaśatvād ekāntaśaityāt kadalīviśeṣāḥ .
    labdhvāpi loke pariṇāhi rūpaṃ jātās tadūrvor upamānabāhyāḥ ..
    etāvatā nanv anumeyaśobhaṃ kāñcīguṇasthānam aninditāyāḥ .
    āropitaṃ yad giriśena paścād ananyanārīkamanīyam aṅkam ..
    tasyāḥ praviṣṭā natanābhirandhraṃ rarāja tanvī navalomarājiḥ .
    nīvīm atikramya sitetarasya tanmekhalāmadhyamaṇer ivārciḥ ..
    madhyena sā vedivilagnamadhyā valitrayaṃ cāru babhāra bālā .
    ārohaṇārthaṃ navayauvanena kāmasya sopānam iva prayuktam ..
    anyonyam utpīḍayad utpalākṣyāḥ stanadvayaṃ pāṇḍu tathā pravṛddham .
    madhye yathā śyāmamukhasya tasya mṛṇālasūtrāntaram apy alabhyam ..
    śirīṣapuṣpādhikasaukumāryau bāhū tadīyāv iti me vitarkaḥ .
    parājitenāpi kṛtau harasya yau kaṇṭhapāśau makaradhvajena ..
    kaṇṭhasya tasyāḥ stanabandhurasya muktākalāpasya ca nistalasya .
    anyonyaśobhājananād babhūva sādhāraṇo bhūṣaṇabhūṣyabhāvaḥ ..
    candraṃ gatā padmaguṇān na bhuṅkte padmāśritā cāndramasīm abhikhyām .
    umāmukhaṃ tu pratipadya lolā dvisaṃśrayāṃ prītim avāpa lakṣmīḥ ..
    puṣpaṃ pravālopahitaṃ yadi syān muktāphalaṃ vā sphuṭavidrumastham .
    tato 'nukuryād viśadasya tasyās tāmrauṣṭhaparyastarucaḥ smitasya ..
    svareṇa tasyām amṛtasruteva prajalpitāyām abhijātavāci .
    apy anyapuṣṭā pratikūlaśabdā śrotur vitantrīr iva tāḍyamānā ..
    pravātanīlotpalanirviśeṣam adhīraviprekṣitam āyatākṣyā .
    tayā gṛhītaṃ nu mṛgāṅganābhyas tato gṛhītaṃ nu mṛgāṅganābhiḥ ..
    tasyāḥ śalākāñjananirmiteva kāntir bhruvor ānatalekhayor yā .
    tāṃ vīkṣya līlācaturām anaṅgaḥ svacāpasaundaryamadaṃ mumoca ..
    lajjā tiraścāṃ yadi cetasi syād asaṃśayaṃ parvatarājaputryāḥ .
    taṃ keśapāśaṃ prasamīkṣya kuryur vālapriyatvaṃ śithilaṃ camaryaḥ ..
    sarvopamādravyasamuccayena yathāpradeśaṃ viniveśitena .
    sā nirmitā viśvasṛjā prayatnād ekasthasaundaryadidṛkṣayeva ..
    tāṃ nāradaḥ kāmacaraḥ kadā cit kanyāṃ kila prekṣya pituḥ samīpe .
    samādideśaikavadhūṃ bhavitrīṃ premṇā śarīrārdhaharāṃ harasya ..
    guruḥ pragalbhe 'pi vayasy ato 'syās tasthau nivṛttānyavarābhilāṣaḥ .
    ṛte kṛśānor na hi mantrapūtam arhanti tejāṃsy aparāṇi havyam ..
    ayācitāraṃ na hi devadevam adriḥ sutāṃ grāhayituṃ śaśāka .
    abhyarthanābhaṅgabhayena sādhur mādhyasthyam iṣṭe 'py avalambate 'rthe ..
    yadaiva pūrve janane śarīraṃ sā dakṣaroṣāt sudatī sasarja .
    tadāprabhṛty eva vimuktasaṅgaḥ patiḥ paśūnām aparigraho 'bhūt ..
    sa kṛttivāsās tapase yatātmā gaṅgāpravāhokṣitadevadāru .
    prasthaṃ himādrer mṛganābhigandhi kiṃ cit kvaṇatkiṃnaram adhyuvāsa ..
    gaṇā nameruprasavāvataṃsā bhūrjatvacaḥ sparśavatīr dadhānāḥ .
    manaḥśilāvicchuritā niṣeduḥ śaileyanaddheṣu śilātaleṣu ..
    tuṣārasaṃghātaśilāḥ khurāgraiḥ samullikhan darpakalaḥ kakudmān .
    dṛṣṭaḥ kathaṃ cid gavayair vivignair asoḍhasiṃhadhvanir unnanāda ..
    tatrāgnim ādhāya samitsamiddhaṃ svam eva mūrtyantaram aṣṭamūrtiḥ .
    svayaṃ vidhātā tapasaḥ phalānām kenāpi kāmena tapaś cacāra ..
    anarghyam arghyeṇa tam adrināthaḥ svargaukasām arcitam arcayitvā .
    ārādhanāyāsya sakhīsametāṃ samādideśa prayatāṃ tanūjām ..
    pratyarthibhūtām api tāṃ samādheḥ śuśrūṣamāṇāṃ giriśo 'numene .
    vikārahetau sati vikriyante yeṣāṃ na cetāṃsi ta eva dhīrāḥ ..
    avacitabalipuṣpā vedisaṃmārgadakṣā niyamavidhijalānāṃ barhiṣāṃ copanetrī .
    giriśam upacacāra pratyahaṃ sā sukeśī niyamitaparikhedā tacchiraścandrapādaiḥ ..".to_string();
    let converted = convert::convert_roman_to_roman(&input, source, destination);
    println!("{}", converted);
}
